use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::http::Status;
use rocket::{request::Outcome, Request};
use rocket_contrib::databases::diesel;

use crate::errors::ServiceError;

pub mod api_token_repo;
pub mod crud_repo;
pub mod pagination;
pub mod problem_repo;
pub mod submission_repo;
pub mod team_repo;
pub mod transaction_manager;
pub mod user_repo;

#[database("diesel_postgres_pool")]
pub struct ConnPool(diesel::PgConnection);

pub type DbConn = PooledConnection<ConnectionManager<diesel::PgConnection>>;

pub fn db_connection_guard(req: &Request) -> Outcome<DbConn, ServiceError> {
    req.guard::<ConnPool>()
        .map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })
        //noinspection RsTypeCheck
        .map(|cp| cp.0)
}
