use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::models::http::responses::{GetProblemsResponse, GetProblemDetailsResponse};
use crate::db::problem_repo::{IProblemRepo, DbProblemRepo};
use crate::models::problem::{ProblemFullyPopulatedDto};
use crate::services::utils_service::UtilsService;
use crate::db::submission_repo::{ISubmissionRepo, DbSubmissionRepo};
use crate::guards::{AccessToken, AuthTokenGuard};

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
            let next_problem = self.problem_repo.find_next_available_problem(now_naive, &td).ok();

            Ok(GetProblemsResponse {
                next_problem_available_at: next_problem.map(|pr| pr.available_from.map_or("".to_string(), |el| el.to_string())),
                problems: available_problems.into_iter().map(|pr| pr.to_dto()).collect(),
            })
        })
    }

    pub fn get_problem_details(&self, user_guard: AuthTokenGuard<AccessToken>, code_name: String) -> anyhow::Result<GetProblemDetailsResponse> {
        self.tm.transaction(|td| {
            let now_naive = UtilsService::naive_now();
            let problem_and_statement = self.problem_repo.find_available_problem_by_code_name_populated(&code_name, now_naive, &td)?;
            let submissions = self.submission_repo.find_submissions_by_user_and_problem(user_guard.user.id, problem_and_statement.0.id, &td)?;

            Ok(GetProblemDetailsResponse {
                problem: ProblemFullyPopulatedDto::from_problem_and_statement(&problem_and_statement.0, &problem_and_statement.1),
                submissions: submissions.into_iter().map(|sub| sub.to_dto()).collect(),
            })
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
