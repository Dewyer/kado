use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};

pub struct SubmissionService {
    tm: TransactionManager,
}

impl SubmissionService {
    pub fn new(
        tm: TransactionManager,
    ) -> Self {
        Self {
            tm,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for SubmissionService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<SubmissionService, Self::Error> {
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(SubmissionService::new(
            db_tm,
        ))
    }
}
