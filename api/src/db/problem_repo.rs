use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::problem::{NewProblem, Problem};
use crate::schema::problems;
use chrono::NaiveDateTime;
use crate::schema::problem_statements;
use crate::models::problem::problem_statement::{ProblemStatement, NewProblemStatement};

crud_repo!(ProblemCrudRepo, DbProblemCrudRepo, problems, Problem, NewProblem, "Problems");
pub type IProblemCrudRepo = Box<dyn ProblemCrudRepo>;

crud_repo!(ProblemStatementCrudRepo, DbProblemStatementCrudRepo, problem_statements, ProblemStatement, NewProblemStatement, "Problem statement");
pub type IProblemStatementCrudRepo = Box<dyn ProblemStatementCrudRepo>;


#[macro_export]
macro_rules! full_availability {
    ($now: ident) => {
        problems::available_from.le($now).or(problems::available_from.is_null()).and(problems::available_until.ge($now).or(problems::available_until.is_null()))
    }
}

pub trait ProblemRepo {
    fn crud(&self) -> &IProblemCrudRepo;

    fn crud_statement(&self) -> &IProblemStatementCrudRepo;

    fn save(&self, problem: &Problem, td: &ITransaction) -> anyhow::Result<Problem>;

    fn find_available_problems(&self, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Vec<Problem>>;

    fn find_next_available_problem(&self, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Problem>;

    fn find_available_problem_by_code(&self, code_name: &str, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Problem>;

    fn find_problem_by_code(&self, code_name: &str, td: &ITransaction) -> anyhow::Result<Problem>;

    fn find_available_problem_by_code_name_populated(&self, code_name: &str, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<(Problem, ProblemStatement)>;
}

pub struct DbProblemRepo {
    crud: IProblemCrudRepo,
    crud_statement: IProblemStatementCrudRepo,
}

impl DbProblemRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbProblemCrudRepo {}),
            crud_statement: Box::new(DbProblemStatementCrudRepo {}),
        }
    }
}

impl ProblemRepo for DbProblemRepo {
    fn crud(&self) -> &IProblemCrudRepo {
        &self.crud
    }

    fn crud_statement(&self) -> &IProblemStatementCrudRepo {
        &self.crud_statement
    }

    fn save(&self, problem: &Problem, td: &ITransaction) -> anyhow::Result<Problem> {
        diesel::update(problems::table)
            .filter(problems::id.eq(problem.id))
            .set(problem)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save problem!"))
    }

    fn find_available_problems(&self, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Vec<Problem>> {
        let full = full_availability!(now);

        problems::table
            .select(problems::all_columns)
            .filter(
                problems::is_deleted.eq(false)
                    .and(full)
            )
            .load::<Problem>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Problems couldn't be loaded!"))
    }

    fn find_next_available_problem(&self, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Problem> {
        problems::table
            .select(problems::all_columns)
            .filter(
                problems::is_deleted.eq(false)
                    .and(problems::available_from.is_not_null().and(problems::available_until.is_not_null().and(problems::available_until.ge(now)).or(problems::available_until.is_null())))
            )
            .order(problems::available_from.desc())
            .first::<Problem>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Next available problem not found!"))
    }

    fn find_available_problem_by_code(&self, code_name: &str, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<Problem> {
        let full = full_availability!(now);
        problems::table
            .filter(
                problems::is_deleted.eq(false)
                    .and(problems::code_name.eq(code_name))
                    .and(full)
            ).first::<Problem>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Problem not found!"))
    }

    fn find_problem_by_code(&self, code_name: &str, td: &ITransaction) -> anyhow::Result<Problem> {
        problems::table
            .filter(
                problems::code_name.eq(code_name)
            ).first::<Problem>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Problem not found!"))
    }

    fn find_available_problem_by_code_name_populated(&self, code_name: &str, now: NaiveDateTime, td: &ITransaction) -> anyhow::Result<(Problem, ProblemStatement)> {
        let full = full_availability!(now);

        problems::table
            .inner_join(problem_statements::table)
            .filter(
                problems::is_deleted.eq(false)
                    .and(problems::code_name.eq(code_name))
                    .and(full)
            ).first::<(Problem, ProblemStatement)>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Problem not found!"))
    }
}

pub type IProblemRepo = Box<dyn ProblemRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbProblemRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbProblemRepo, Self::Error> {
        request::Outcome::Success(DbProblemRepo::new())
    }
}
