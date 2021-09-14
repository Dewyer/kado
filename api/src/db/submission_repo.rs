use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::submission::{NewSubmission, Submission};
use crate::schema::submissions;
use chrono::NaiveDateTime;
use crate::schema::submission_statements;
use crate::models::submission::submission_statement::SubmissionStatement;
use uuid::Uuid;

crud_repo!(SubmissionCrudRepo, DbSubmissionCrudRepo, submissions, Submission, NewSubmission, "Submissions");
pub type ISubmissionCrudRepo = Box<dyn SubmissionCrudRepo>;

pub trait SubmissionRepo {
    fn crud(&self) -> &ISubmissionCrudRepo;

    fn save(&self, submission: &Submission, td: &ITransaction) -> anyhow::Result<Submission>;

    fn find_submissions_by_user_and_problem(&self, user_id: Uuid, problem_id: Uuid, td: &ITransaction) -> anyhow::Result<Vec<Submission>>;
}

pub struct DbSubmissionRepo {
    crud: ISubmissionCrudRepo,
}

impl DbSubmissionRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbSubmissionCrudRepo {}),
        }
    }
}

impl SubmissionRepo for DbSubmissionRepo {
    fn crud(&self) -> &ISubmissionCrudRepo {
        &self.crud
    }

    fn save(&self, submission: &Submission, td: &ITransaction) -> anyhow::Result<Submission> {
        diesel::update(submissions::table)
            .filter(submissions::id.eq(submission.id))
            .set(submission)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save submission!"))
    }

    fn find_submissions_by_user_and_problem(&self, user_id: Uuid, problem_id: Uuid, td: &ITransaction) -> anyhow::Result<Vec<Submission>> {
        submissions::table
            .select(submissions::all_columns)
            .filter(
                submissions::owner_id.eq(user_id).and(submissions::problem_id.eq(problem_id)),
            )
            .load::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submissions couldn't be loaded!"))
    }
}

pub type ISubmissionRepo = Box<dyn SubmissionRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbSubmissionRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbSubmissionRepo, Self::Error> {
        request::Outcome::Success(DbSubmissionRepo::new())
    }
}
