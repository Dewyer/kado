use crate::db::transaction_manager::{TransactionManager, ITransaction};
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::guards::{AuthTokenGuard, ApiToken};
use crate::models::http::requests::{StartSubmissionRequest, SendTestOutputRequest, GetTestInputRequest};
use crate::db::problem_repo::{IProblemRepo, DbProblemRepo};
use crate::services::utils_service::UtilsService;
use crate::models::problem::code_name::CodeName;
use crate::models::problem::Problem;
use crate::models::user::User;
use crate::models::submission::{Submission, NewSubmission};
use crate::db::submission_repo::{ISubmissionRepo, DbSubmissionRepo};
use crate::services::submission::support::{IProblemSupport, CamelCaseProblemSupport, SubmissionGenerationPayload, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload};
use rand::Rng;
use crate::models::http::responses::{StartSubmissionResponse, GetTestInputResponse, SendTestOutputResponse};
use crate::models::submission::submission_test::{SubmissionTest, NewSubmissionTest};
use crate::db::team_repo::{DbTeamRepo, ITeamRepo};
use uuid::Uuid;
use crate::db::user_repo::{IUserRepo, DbUserRepo};

const SUBMISSION_TEST_TIMEOUT: i64 = 1000000;

pub struct SubmissionService {
    submission_repo: ISubmissionRepo,
    problem_repo: IProblemRepo,
    user_repo: IUserRepo,
    team_repo: ITeamRepo,
    tm: TransactionManager,
}

impl SubmissionService {
    pub fn new(
        submission_repo: ISubmissionRepo,
        problem_repo: IProblemRepo,
        user_repo: IUserRepo,
        team_repo: ITeamRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            submission_repo,
            problem_repo,
            user_repo,
            team_repo,
            tm,
        }
    }

    fn get_problem_support(&self, code_name: CodeName) -> IProblemSupport {
        match code_name {
            CodeName::CamelCase => Box::new(CamelCaseProblemSupport::new()),
        }
    }

    fn create_submission(&self, user: &User, problem: &Problem, request: &StartSubmissionRequest, td: &ITransaction) -> anyhow::Result<Submission> {
        let now_naive = UtilsService::naive_now();
        let code_name = CodeName::from_string(&request.problem)?;
        let support = self.get_problem_support(code_name.clone());
        let seed = rand::thread_rng().gen_range(0, 1000000);
        let gen_payload = SubmissionGenerationPayload {
            seed,
        };
        let gen_res = support.generate_submission_details(gen_payload)?;

        self.submission_repo.crud().insert(&NewSubmission {
            owner_id: user.id,
            problem_id: problem.id,
            seed,
            test_count: gen_res.test_count,
            sample_index: request.sample_index,
            correct: None,
            started_at: now_naive,
            finished_at: None,
        }, td)
    }

    fn assert_can_start_submission(&self, user: &User, problem: &Problem, request: &StartSubmissionRequest, td: &ITransaction) -> anyhow::Result<()> {
        if problem.sample_count-1 < request.sample_index.unwrap_or(0) {
            bail!("Sample index is larger then the number of samples available");
        }

        let submissions_made = self.submission_repo.find_submissions_by_user_and_problem(user.id, problem.id, td)?;
        if submissions_made.len() >= problem.max_submissions as usize {
            bail!("Max submission attempts reached.");
        }

        let open_submission = submissions_made.iter().find(|sb| sb.finished_at.is_none());
        if open_submission.is_some() {
            bail!("There is an open submission made by you for this problem, finish that or let it time out first.");
        }

        if submissions_made.iter().any(|sub| sub.correct.contains(&true)) {
            bail!("You already have a correct submission for this problem");
        }

        Ok(())
    }

    pub fn start_submission(&self, user_guard: AuthTokenGuard<ApiToken>, request: StartSubmissionRequest) -> anyhow::Result<StartSubmissionResponse> {
        self.tm.transaction(|td| {
            let now_naive = UtilsService::naive_now();
            let problem = self.problem_repo.find_available_problem_by_code(&request.problem, now_naive, &td)?;
            self.assert_can_start_submission(&user_guard.user, &problem, &request, &td)?;

            let new_submission = self.create_submission(&user_guard.user, &problem, &request, &td)?;

            Ok(StartSubmissionResponse {
                submission: new_submission.to_dto(),
            })
        })
    }

    fn assert_can_start_new_test(&self, submission: &Submission, existing_tests: &Vec<SubmissionTest>) -> anyhow::Result<()> {
        let open_test = existing_tests.iter().find(|test| test.finished_at.is_none());
        if open_test.is_some() {
            bail!("You already have an open test running, submit it before getting new inputs!");
        }

        if existing_tests.len() >= submission.test_count as usize {
            bail!("You already submitted all the required tests!");
        }

        Ok(())
    }

    fn create_submission_test(&self, problem: &Problem, submission: &Submission, existing_tests: &Vec<SubmissionTest>, td: &ITransaction) -> anyhow::Result<(SubmissionTest, SubmissionTestGenerationResult)> {
        let now_naive = UtilsService::naive_now();
        let code_name = CodeName::from_string(&problem.code_name)?;
        let support = self.get_problem_support(code_name.clone());
        let input_res = support.generate_submission_test_input(SubmissionTestGenerationPayload {
            test_index: existing_tests.len(),
            seed: submission.seed,
        })?;

        Ok((
            self.submission_repo.crud_tests().insert(&NewSubmissionTest {
                submission_id: submission.id,
                class: &input_res.test_class,
                input: &input_res.input.to_string(),
                output: None,
                correct: None,
                started_at: now_naive,
                finished_at: None,
            }, td)?,
            input_res
        ))
    }

    pub fn get_test_input(&self, user_guard: AuthTokenGuard<ApiToken>, request: GetTestInputRequest) -> anyhow::Result<GetTestInputResponse> {
        self.tm.transaction(|td| {
            let code_name = request.problem;
            let user = user_guard.user;
            let now_naive = UtilsService::naive_now();
            CodeName::from_string(&code_name)?;

            let problem = self.problem_repo.find_available_problem_by_code(&code_name, now_naive, &td)?;
            let (submission, submission_tests) = self.submission_repo.find_latest_submission_by_user_and_problem_with_tests(user.id, problem.id, &td)?;
            self.assert_can_start_new_test(&submission, &submission_tests)?;

            let (new_test, generated_test) = self.create_submission_test(&problem, &submission, &submission_tests, &td)?;

            Ok(GetTestInputResponse {
                test_id: new_test.id.to_string(),
                deadline: now_naive.timestamp() + SUBMISSION_TEST_TIMEOUT,
                input: generated_test.input,
            })
        })
    }

    fn assert_can_submit_test_output(&self, test: &SubmissionTest) -> anyhow::Result<()> {
        if !UtilsService::time_within_seconds(test.started_at, SUBMISSION_TEST_TIMEOUT) {
            bail!("Test submission timed out!");
        }

        Ok(())
    }

    fn get_diminishing_returns_on_points(&self, max_p: i64, sub_count: usize) -> i64 {
        const R: f64 = 0.2;
        (max_p * std::f64::consts::E.powf(-R * (sub_count as f64))).floor() as i64
    }

    fn handle_team_member_submission_completion(&self, user: &User, team_id: Uuid, problem: &Problem, td: &ITransaction) -> anyhow::Result<()> {
        let (mut team, team_members) = self.team_repo.find_by_id_with_users(team_id, td)?;
        let submissions_for_team_members = self.submission_repo.find_correct_submissions_for_users_and_problem(team_members.iter().map(|usr| usr.id).collect(), problem.id, td)?;
        team.points += self.get_diminishing_returns_on_points(problem.base_point_value, submissions_for_team_members.len());

        self.team_repo.save(&team, td)?;
        Ok(())
    }

    fn handle_submission_finished(&self, mut submission: Submission, problem: &Problem, mut user: User, existing_tests: &Vec<SubmissionTest>, updated_last_test: &SubmissionTest, td: &ITransaction) -> anyhow::Result<()> {
        submission.finished_at = updated_last_test.finished_at;
        let existing_tests_without_last = existing_tests.iter().filter(|test| test.id != updated_last_test.id).collect::<Vec<&SubmissionTest>>();
        if existing_tests_without_last.iter().any(|test| !test.correct.unwrap_or(false)) || !updated_last_test.correct.unwrap_or(false) {
            submission.correct = Some(false);
            self.submission_repo.save(&submission, td)?;

            return Ok(());
        }
        submission.correct = Some(true);

        self.submission_repo.save(&submission, td)?;

        user.individual_points += problem.base_point_value;
        if let Some(team_id) = user.team_id {
            self.handle_team_member_submission_completion(&user, team_id, problem, td)?;
        }

        self.user_repo.save(&user, td)?;
        Ok(())
    }

    fn submit_test_output(&self, mut test: SubmissionTest, request: &SendTestOutputRequest, user: User, td: &ITransaction) -> anyhow::Result<SubmissionTest> {
        let (submission, existing_tests) = self.submission_repo.find_by_id_with_tests(test.submission_id, td)?;
        let problem = self.problem_repo.crud().find_by_id(submission.problem_id, td)?;
        let code_name = CodeName::from_string(&problem.code_name)?;

        let support = self.get_problem_support(code_name.clone());
        let verification_result = support.verify_output(VerificationPayload {
            test: &test,
            output: &request.output,
        })?;

        test.correct = Some(verification_result.correct);
        test.finished_at = Some(UtilsService::naive_now());
        test.output = Some(request.output.to_string());
        self.submission_repo.save_test(&test, td)?;

        if submission.test_count as usize == existing_tests.len() {
            self.handle_submission_finished(submission, &problem, user, &existing_tests, &test, td)?;
        }

        Ok(test)
    }

    pub fn send_test_output(&self, user_guard: AuthTokenGuard<ApiToken>, test_id: String, request: SendTestOutputRequest) -> anyhow::Result<SendTestOutputResponse> {
        self.tm.transaction(|td| {
            let test_uuid = UtilsService::parse_uuid(&test_id)?;
            let test = self.submission_repo.crud_tests().find_by_id(test_uuid, &td)?;
            self.assert_can_submit_test_output(&test)?;

            let new_test = self.submit_test_output(test, &request, user_guard.user, &td)?;

            Ok(SendTestOutputResponse {
                correct: new_test.correct.unwrap_or(false),
            })
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for SubmissionService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<SubmissionService, Self::Error> {
        let submission_repo = req.guard::<DbSubmissionRepo>()?;
        let user_repo = req.guard::<DbUserRepo>()?;
        let team_repo = req.guard::<DbTeamRepo>()?;
        let problem_repo = req.guard::<DbProblemRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(SubmissionService::new(
            Box::new(submission_repo),
            Box::new(problem_repo),
            Box::new(user_repo),
            Box::new(team_repo),
            db_tm,
        ))
    }
}
