use crate::crud_repo;
use crate::db::pagination::*;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::team::{NewTeam, Team};
use crate::models::user::User;
use crate::models::utils::PaginationOptions;
use crate::schema::{teams, users};
use diesel::dsl::count_star;
use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};
use uuid::Uuid;

crud_repo!(TeamCrudRepo, DbTeamCrudRepo, teams, Team, NewTeam, "Teams");
pub type ITeamCrudRepo = Box<dyn TeamCrudRepo>;

pub trait TeamRepo {
    fn crud(&self) -> &ITeamCrudRepo;

    fn find_by_join_code(&self, join_code: &str, td: &ITransaction) -> anyhow::Result<Team>;

    fn find_by_name(&self, name: &str, td: &ITransaction) -> anyhow::Result<Team>;

    fn find_by_id_not_deleted(&self, id: Uuid, td: &ITransaction) -> anyhow::Result<Team>;

    fn find_by_id_with_users(
        &self,
        id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Team, Vec<User>)>;

    fn save(&self, team: &Team, td: &ITransaction) -> anyhow::Result<Team>;

    fn list_leaderboard_participating_paginated(
        &self,
        pagination: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<Team>, usize)>;

    fn find_leaderboard_rank_of_team(
        &self,
        team: &Team,
        td: &ITransaction,
    ) -> anyhow::Result<usize>;
}

pub struct DbTeamRepo {
    crud: ITeamCrudRepo,
}

impl DbTeamRepo {
    pub fn new() -> Self {
        Self {
            crud: Box::new(DbTeamCrudRepo {}),
        }
    }
}

pub const DEFAULT_USER_PAGE_SIZE: usize = 25;

impl TeamRepo for DbTeamRepo {
    fn crud(&self) -> &ITeamCrudRepo {
        &self.crud
    }

    fn find_by_join_code(&self, join_code: &str, td: &ITransaction) -> anyhow::Result<Team> {
        teams::table
            .select(teams::all_columns)
            .filter(
                teams::join_code
                    .eq(join_code)
                    .and(teams::is_deleted.eq(false)),
            )
            .first::<Team>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Team not found!"))
    }

    fn find_by_name(&self, name: &str, td: &ITransaction) -> anyhow::Result<Team> {
        teams::table
            .select(teams::all_columns)
            .filter(teams::name.eq(name).and(teams::is_deleted.eq(false)))
            .first::<Team>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Team not found!"))
    }

    fn find_by_id_not_deleted(&self, id: Uuid, td: &ITransaction) -> anyhow::Result<Team> {
        teams::table
            .select(teams::all_columns)
            .filter(teams::id.eq(id).and(teams::is_deleted.eq(false)))
            .first::<Team>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Team not found!"))
    }

    fn find_by_id_with_users(
        &self,
        id: Uuid,
        td: &ITransaction,
    ) -> anyhow::Result<(Team, Vec<User>)> {
        let team = self.find_by_id_not_deleted(id, td)?;
        let users_on_team: Vec<User> = users::table
            .select(users::all_columns)
            .filter(users::team_id.eq(Some(id)))
            .load::<User>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Users for team could not be found!"))?;

        Ok((team, users_on_team))
    }

    fn save(&self, team: &Team, td: &ITransaction) -> anyhow::Result<Team> {
        diesel::update(teams::table)
            .filter(teams::id.eq(team.id))
            .set(team)
            .get_result(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Can't save team!"))
    }

    fn list_leaderboard_participating_paginated(
        &self,
        pagination: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<Team>, usize)> {
        teams::table
            .order((
                teams::points.desc(),
                teams::last_gained_points_at.asc().nulls_last(),
                teams::created_at.asc(),
            ))
            .filter(teams::is_deleted.eq(false))
            .paginate((pagination.page + 1) as i64)
            .per_page(std::cmp::min(
                pagination.per_page.unwrap_or(DEFAULT_USER_PAGE_SIZE),
                DEFAULT_USER_PAGE_SIZE,
            ) as i64)
            .load_and_count_pages::<Team>(td.get_db_connection())
            .map(|(vv, nn)| (vv, nn as usize))
            .map_err(|_| anyhow::Error::msg("Can't load leaderboard!"))
    }

    fn find_leaderboard_rank_of_team(
        &self,
        team: &Team,
        td: &ITransaction,
    ) -> anyhow::Result<usize> {
        teams::table
            .filter(
                teams::is_deleted.eq(false).and(
                    teams::points.ge(team.points).or(teams::points
                        .eq(&team.points)
                        .and(teams::last_gained_points_at.le(&team.last_gained_points_at))
                        .or(teams::last_gained_points_at
                            .eq(&team.last_gained_points_at)
                            .and(teams::created_at.le(&team.created_at)))),
                ),
            )
            .select(count_star())
            .first::<i64>(td.get_db_connection())
            .map(|vv| (vv - 1) as usize)
            .map_err(|_| anyhow::Error::msg("Can't load team's rank!"))
    }
}

pub type ITeamRepo = Box<dyn TeamRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbTeamRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbTeamRepo, Self::Error> {
        request::Outcome::Success(DbTeamRepo::new())
    }
}
