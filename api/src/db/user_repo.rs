use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::user::db::{NewUser, User};
use crate::schema::users;

crud_repo!(UserCrudRepo, DbUserCrudRepo, users, User, NewUser, "Users");
pub type IUserCrudRepo = Box<dyn UserCrudRepo>;

pub trait UserRepo {
    fn crud(&self) -> &IUserCrudRepo;

    fn create(&self, user: &NewUser, tr: &ITransaction) -> anyhow::Result<User>;

    fn find_by_email_or_username(
        &self,
        email: &str,
        username: &str,
        tm: &ITransaction,
    ) -> anyhow::Result<User>;

    fn save(&self, user: &User, td: &ITransaction) -> anyhow::Result<User>;
}

pub struct DbUserRepo {
    crud: IUserCrudRepo,
}

impl DbUserRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbUserCrudRepo {}),
        }
    }
}

impl UserRepo for DbUserRepo {
    fn crud(&self) -> &IUserCrudRepo {
        &self.crud
    }

    fn create(&self, user: &NewUser, tr: &ITransaction) -> anyhow::Result<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(tr.get_db_connection())
            .map_err(|er| anyhow::Error::from(er))
    }

    fn find_by_email_or_username(
        &self,
        email: &str,
        username: &str,
        tr: &ITransaction,
    ) -> anyhow::Result<User> {
        users::table
            .select(users::all_columns)
            .filter(users::email.eq(email))
            .or_filter(users::username.eq(username))
            .first::<User>(tr.get_db_connection())
            .map_err(|er| anyhow::Error::from(er))
    }

    fn save(&self, user: &User, td: &ITransaction) -> anyhow::Result<User> {
        diesel::update(users::table)
            .filter(users::id.eq(user.id))
            .set(user)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save user!"))
    }
}

pub type IUserRepo = Box<dyn UserRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbUserRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbUserRepo, Self::Error> {
        request::Outcome::Success(DbUserRepo::new())
    }
}
