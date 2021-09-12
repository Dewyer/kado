use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};

pub struct ProblemService {
    tm: TransactionManager,
}


impl ProblemService {
    pub fn new(
        tm: TransactionManager,
    ) -> Self {
        Self {
            tm,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ProblemService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<ProblemService, Self::Error> {
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(ProblemService::new(
            db_tm,
        ))
    }
}
