use crate::db::problem_repo::{DbProblemRepo, IProblemRepo};
use crate::db::submission_repo::{DbSubmissionRepo, ISubmissionRepo};
use crate::db::transaction_manager::TransactionManager;
use crate::errors::ServiceError;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::requests::UpdateProblemStatementRequest;
use crate::models::http::responses::{GetProblemDetailsResponse, GetProblemsResponse};
use crate::models::problem::code_name::CodeName;
use crate::models::problem::problem_statement::NewProblemStatement;
use crate::models::problem::ProblemFullyPopulatedDto;
use crate::services::utils_service::UtilsService;
use rocket::request::FromRequest;
use rocket::{request, Request};

pub struct ProblemService {
    problem_repo: IProblemRepo,
    submission_repo: ISubmissionRepo,
    tm: TransactionManager,
}

impl ProblemService {
    pub fn new(
        problem_repo: IProblemRepo,
        submission_repo: ISubmissionRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            problem_repo,
            submission_repo,
            tm,
        }
    }

    pub fn get_problems(&self) -> anyhow::Result<GetProblemsResponse> {
        self.tm.transaction(|td| {
            let now_naive = UtilsService::naive_now();
            let available_problems = self.problem_repo.find_available_problems(now_naive, &td)?;
            let next_problem = self
                .problem_repo
                .find_next_available_problem(now_naive, &td)
                .ok();

            Ok(GetProblemsResponse {
                next_problem_available_at: next_problem.map(|pr| {
                    pr.available_from
                        .map_or("".to_string(), |el| el.to_string())
                }),
                problems: available_problems
                    .into_iter()
                    .map(|pr| pr.to_dto())
                    .collect(),
            })
        })
    }

    pub fn get_problem_details(
        &self,
        user_guard: AuthTokenGuard<AccessToken>,
        code_name: String,
    ) -> anyhow::Result<GetProblemDetailsResponse> {
        self.tm.transaction(|td| {
            let now_naive = UtilsService::naive_now();
            let problem_and_statement = self
                .problem_repo
                .find_available_problem_by_code_name_populated(&code_name, now_naive, &td)?;
            let submissions = self.submission_repo.find_submissions_by_user_and_problem(
                user_guard.user.id,
                problem_and_statement.0.id,
                &td,
            )?;

            Ok(GetProblemDetailsResponse {
                problem: ProblemFullyPopulatedDto::from_problem_and_statement(
                    &problem_and_statement.0,
                    &problem_and_statement.1,
                ),
                submissions: submissions.into_iter().map(|sub| sub.to_dto()).collect(),
            })
        })
    }

    pub fn update_problem_statement_by_admin(
        &self,
        request: UpdateProblemStatementRequest,
    ) -> anyhow::Result<()> {
        self.tm.transaction(|td| {
            CodeName::from_string(&request.problem)?;
            let mut problem = self
                .problem_repo
                .find_problem_by_code(&request.problem, &td)?;
            let statement = self
                .problem_repo
                .crud_statement()
                .find_by_id(problem.problem_statement_id, &td)?;

            if statement.problem_statement == request.statement {
                return Ok(());
            }

            let new_statement = self.problem_repo.crud_statement().insert(
                &NewProblemStatement {
                    problem_statement: &request.statement,
                    version: &request.version,
                },
                &td,
            )?;
            problem.problem_statement_id = new_statement.id;
            self.problem_repo.save(&problem, &td)?;

            Ok(())
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ProblemService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<ProblemService, Self::Error> {
        let problem_repo = req.guard::<DbProblemRepo>()?;
        let submission_repo = req.guard::<DbSubmissionRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(ProblemService::new(
            Box::new(problem_repo),
            Box::new(submission_repo),
            db_tm,
        ))
    }
}
