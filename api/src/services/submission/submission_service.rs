use crate::db::transaction_manager::{TransactionManager, ITransaction};
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::guards::{AuthTokenGuard, ApiToken};
use crate::models::http::requests::StartSubmissionRequest;
use crate::db::problem_repo::{IProblemRepo, DbProblemRepo};
use crate::services::utils_service::UtilsService;
use crate::models::problem::code_name::CodeName;
use crate::models::problem::Problem;
use crate::models::user::User;
use crate::models::submission::{Submission, NewSubmission};
use crate::db::submission_repo::{ISubmissionRepo, DbSubmissionRepo};
use crate::services::submission::support::{IProblemSupport, CamelCaseProblemSupport, SubmissionGenerationPayload, SubmissionTestGenerationPayload, SubmissionTestGenerationResult};
use rand::Rng;
use crate::models::http::responses::{StartSubmissionResponse, GetTestInputResponse};
use crate::models::submission::submission_test::{SubmissionTest, NewSubmissionTest};

pub struct SubmissionService {
    submission_repo: ISubmissionRepo,
    problem_repo: IProblemRepo,
    tm: TransactionManager,
}

impl SubmissionService {
    pub fn new(
        submission_repo: ISubmissionRepo,
        problem_repo: IProblemRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            submission_repo,
            problem_repo,
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
        let code_name = CodeName::from_string(&request.problem_id)?;
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
            started_at: now_naive,
            finished_at: None,
        }, td)
    }

    fn assert_can_start_submission(&self,user: &User, problem: &Problem, request: &StartSubmissionRequest, td: &ITransaction) -> anyhow::Result<()> {
        if problem.sample_count >= request.sample_index.unwrap_or(0) {
            bail!("Sample index is larger then the number of samples available");
        }

        let submissions_made = self.submission_repo.find_submissions_by_user_and_problem(user.id, problem.id, td)?;
        if submissions_made.len() >= problem.max_submissions as usize {
            bail!("Max submission attempts reached.")
        }

        let open_submission = submissions_made.iter().find(|sb| sb.finished_at.is_none());
        if open_submission.is_some() {
            bail!("There is an open submission made by you for this problem, finish that or let it time out first.");
        }

        Ok(())
    }

    pub fn start_submission(&self, user_guard: AuthTokenGuard<ApiToken>, request: StartSubmissionRequest) -> anyhow::Result<StartSubmissionResponse> {
        self.tm.transaction(|td| {
            let now_naive = UtilsService::naive_now();
            let problem = self.problem_repo.find_available_problem_by_code(&request.problem_id, now_naive, &td)?;
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

    pub fn get_test_input(&self, user_guard: AuthTokenGuard<ApiToken>, code_name: String) -> anyhow::Result<GetTestInputResponse> {
        self.tm.transaction(|td| {
            let user = user_guard.user;
            let now_naive = UtilsService::naive_now();
            CodeName::from_string(&code_name)?;

            let problem = self.problem_repo.find_available_problem_by_code(&code_name, now_naive, &td)?;
            let (submission, submission_tests) = self.submission_repo.find_latest_submission_by_user_and_problem_with_tests(user.id, problem.id, &td)?;
            self.assert_can_start_new_test(&submission, &submission_tests)?;

            let (_, generated_test) = self.create_submission_test(&problem, &submission, &submission_tests, &td)?;

            Ok(GetTestInputResponse {
                input: generated_test.input,
            })
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for SubmissionService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<SubmissionService, Self::Error> {
        let submission_repo = req.guard::<DbSubmissionRepo>()?;
        let problem_repo = req.guard::<DbProblemRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(SubmissionService::new(
            Box::new(submission_repo),
            Box::new(problem_repo),
            db_tm,
        ))
    }
}
