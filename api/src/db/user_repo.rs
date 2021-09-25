use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::pagination::*;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::team::Team;
use crate::models::user::db::{NewUser, User};
use crate::models::utils::PaginationOptions;
use crate::schema::teams;
use crate::schema::users;
use diesel::dsl::count_star;

crud_repo!(UserCrudRepo, DbUserCrudRepo, users, User, NewUser, "Users");
pub type IUserCrudRepo = Box<dyn UserCrudRepo>;

pub const DEFAULT_USER_PAGE_SIZE: usize = 25;

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

    fn list_leaderboard_participating_paginated_with_teams(
        &self,
        pagination: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<(User, Option<Team>)>, usize)>;

    fn find_leaderboard_rank_of_user(
        &self,
        user: &User,
        td: &ITransaction,
    ) -> anyhow::Result<usize>;
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

    fn list_leaderboard_participating_paginated_with_teams(
        &self,
        pagination: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<(User, Option<Team>)>, usize)> {
        users::table
            .order((
                users::individual_points.desc(),
                users::last_gained_points_at.asc().nulls_last(),
                users::created_at.asc(),
            ))
            .filter(users::is_active.eq(true).and(users::is_admin.eq(false)))
            .left_join(teams::table)
            .paginate((pagination.page + 1) as i64)
            .per_page(std::cmp::min(
                pagination.per_page.unwrap_or(DEFAULT_USER_PAGE_SIZE),
                DEFAULT_USER_PAGE_SIZE,
            ) as i64)
            .load_and_count_pages::<(User, Option<Team>)>(td.get_db_connection())
            .map(|(vv, nn)| (vv, nn as usize))
            .map_err(|_| anyhow::Error::msg("Can't load leaderboard!"))
    }

    fn find_leaderboard_rank_of_user(
        &self,
        user: &User,
        td: &ITransaction,
    ) -> anyhow::Result<usize> {
        users::table
            .filter(
                users::is_active.eq(true).and(users::is_admin.eq(false)).and(
                    users::individual_points.ge(user.individual_points).or(
                        users::individual_points
                            .eq(&user.individual_points)
                            .and(users::last_gained_points_at.le(&user.last_gained_points_at))
                            .or(users::last_gained_points_at
                                .eq(&user.last_gained_points_at)
                                .and(users::created_at.le(&user.created_at))),
                    ),
                ),
            )
            .select(count_star())
            .first::<i64>(td.get_db_connection())
            .map(|vv| (vv - 1) as usize)
            .map_err(|_| anyhow::Error::msg("Can't load user's rank!"))
    }
}

pub type IUserRepo = Box<dyn UserRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbUserRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbUserRepo, Self::Error> {
        request::Outcome::Success(DbUserRepo::new())
    }
}
