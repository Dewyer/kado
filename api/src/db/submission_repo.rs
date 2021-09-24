use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::submission::submission_test::{NewSubmissionTest, SubmissionTest};
use crate::models::submission::{NewSubmission, Submission};
use crate::schema::submission_tests;
use crate::schema::submissions;
use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};
use uuid::Uuid;

crud_repo!(
    SubmissionCrudRepo,
    DbSubmissionCrudRepo,
    submissions,
    Submission,
    NewSubmission,
    "Submissions"
);
crud_repo!(
    SubmissionTestCrudRepo,
    DbSubmissionTestCrudRepo,
    submission_tests,
    SubmissionTest,
    NewSubmissionTest,
    "SubmissionTests"
);

pub type ISubmissionCrudRepo = Box<dyn SubmissionCrudRepo>;
pub type ISubmissionTestCrudRepo = Box<dyn SubmissionTestCrudRepo>;

pub trait SubmissionRepo {
    fn crud(&self) -> &ISubmissionCrudRepo;

    fn crud_tests(&self) -> &ISubmissionTestCrudRepo;

    fn save(&self, submission: &Submission, td: &ITransaction) -> anyhow::Result<Submission>;

    fn save_test(&self, test: &SubmissionTest, td: &ITransaction)
        -> anyhow::Result<SubmissionTest>;

    fn find_submissions_by_user_and_problem(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Vec<Submission>>;

    fn find_latest_submission_by_user_and_problem_with_tests(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Submission, Vec<SubmissionTest>)>;

    fn find_by_id_with_tests(
        &self,
        submission_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Submission, Vec<SubmissionTest>)>;

    fn find_correct_submissions_for_users_and_problem(
        &self,
        user_ids: Vec<Uuid>,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Vec<Submission>>;

    fn find_correct_submission_for_user_and_problem(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Submission>;
}

pub struct DbSubmissionRepo {
    crud_tests: ISubmissionTestCrudRepo,
    crud: ISubmissionCrudRepo,
}

impl DbSubmissionRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbSubmissionCrudRepo {}),
            crud_tests: Box::new(DbSubmissionTestCrudRepo {}),
        }
    }

    fn find_submission_tests_by_submission(
        &self,
        submission_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Vec<SubmissionTest>> {
        submission_tests::table
            .select(submission_tests::all_columns)
            .filter(submission_tests::submission_id.eq(submission_id))
            .load::<SubmissionTest>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submission tests couldn't be loaded!"))
    }
}

impl SubmissionRepo for DbSubmissionRepo {
    fn crud(&self) -> &ISubmissionCrudRepo {
        &self.crud
    }

    fn crud_tests(&self) -> &ISubmissionTestCrudRepo {
        &self.crud_tests
    }

    fn save(&self, submission: &Submission, td: &ITransaction) -> anyhow::Result<Submission> {
        diesel::update(submissions::table)
            .filter(submissions::id.eq(submission.id))
            .set(submission)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save submission!"))
    }

    fn save_test(
        &self,
        test: &SubmissionTest,
        td: &ITransaction,
    ) -> anyhow::Result<SubmissionTest> {
        diesel::update(submission_tests::table)
            .filter(submission_tests::id.eq(test.id))
            .set(test)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save submission test!"))
    }

    fn find_submissions_by_user_and_problem(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Vec<Submission>> {
        submissions::table
            .select(submissions::all_columns)
            .filter(
                submissions::owner_id
                    .eq(user_id)
                    .and(submissions::problem_id.eq(problem_id)),
            )
            .load::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submissions couldn't be loaded!"))
    }

    fn find_latest_submission_by_user_and_problem_with_tests(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Submission, Vec<SubmissionTest>)> {
        let submission = submissions::table
            .select(submissions::all_columns)
            .filter(
                submissions::owner_id
                    .eq(user_id)
                    .and(submissions::problem_id.eq(problem_id))
                    .and(submissions::finished_at.is_null()),
            )
            .first::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submission not found!!"))?;

        let tests = self.find_submission_tests_by_submission(submission.id, td)?;

        Ok((submission, tests))
    }

    fn find_by_id_with_tests(
        &self,
        submission_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Submission, Vec<SubmissionTest>)> {
        let submission = submissions::table
            .select(submissions::all_columns)
            .filter(submissions::id.eq(submission_id))
            .first::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submission not found!!"))?;

        let tests = self.find_submission_tests_by_submission(submission.id, td)?;

        Ok((submission, tests))
    }

    fn find_correct_submissions_for_users_and_problem(
        &self,
        user_ids: Vec<Uuid>,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Vec<Submission>> {
        submissions::table
            .select(submissions::all_columns)
            .filter(
                submissions::owner_id
                    .eq_any(user_ids)
                    .and(submissions::correct.eq(true))
                    .and(submissions::problem_id.eq(problem_id)),
            )
            .load::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Submissions couldn't be loaded!"))
    }

    fn find_correct_submission_for_user_and_problem(
        &self,
        user_id: Uuid,
        problem_id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<Submission> {
        submissions::table
            .select(submissions::all_columns)
            .filter(
                submissions::owner_id
                    .eq(user_id)
                    .and(submissions::correct.eq(true))
                    .and(submissions::problem_id.eq(problem_id)),
            )
            .first::<Submission>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Correct submission couldn't be found!"))
    }
}

pub type ISubmissionRepo = Box<dyn SubmissionRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbSubmissionRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbSubmissionRepo, Self::Error> {
        request::Outcome::Success(DbSubmissionRepo::new())
    }
}
