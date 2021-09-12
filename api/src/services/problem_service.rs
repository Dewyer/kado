use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::models::http::responses::GetProblemsResponse;
use crate::db::problem_repo::{IProblemRepo, DbProblemRepo};
use chrono::{NaiveDateTime, DateTime, Utc};

pub struct ProblemService {
    problem_repo: IProblemRepo,
    tm: TransactionManager,
}


impl ProblemService {
    pub fn new(
        problem_repo: IProblemRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            problem_repo,
            tm,
        }
    }

    pub fn get_problems(&self) -> anyhow::Result<GetProblemsResponse> {
        self.tm.transaction(|td| {
            let now = Utc::now();
            let now_naive = NaiveDateTime::from_timestamp(now.timestamp(),0);
            let available_problems = self.problem_repo.find_available_problems(now_naive)?;
            let next_problem = self.problem_repo.find_next_available_problem(now_naive).ok();

            Ok(GetProblemsResponse {
                next_problem_available_at: None,
                problems: vec![],
            })
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ProblemService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<ProblemService, Self::Error> {
        let problem_repo = req.guard::<DbProblemRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(ProblemService::new(
            Box::new(problem_repo),
            db_tm,
        ))
    }
}
