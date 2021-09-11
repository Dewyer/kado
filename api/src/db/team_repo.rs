use diesel::prelude::*;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::crud_repo;
use crate::db::transaction_manager::ITransaction;
use crate::errors::ServiceError;
use crate::models::team::{Team, NewTeam};
use crate::schema::{teams, users};
use crate::models::user::User;
use uuid::Uuid;

crud_repo!(TeamCrudRepo, DbTeamCrudRepo, teams, Team, NewTeam, "Teams");
pub type ITeamCrudRepo = Box<dyn TeamCrudRepo>;

pub trait TeamRepo {
    fn crud(&self) -> &ITeamCrudRepo;

    fn find_by_name(&self, name: &str, td: &ITransaction) -> anyhow::Result<Team>;

    fn find_by_id_not_deleted(&self, id: Uuid, td: &ITransaction) -> anyhow::Result<Team>;

    fn find_by_id_with_users(&self, id: Uuid, td: &ITransaction) -> anyhow::Result<(Team, Vec<User>)>;
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

impl TeamRepo for DbTeamRepo {
    fn crud(&self) -> &ITeamCrudRepo {
        &self.crud
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

    fn find_by_id_with_users(&self, id: Uuid, td: &ITransaction) -> anyhow::Result<(Team, Vec<User>)> {
        let team = self.find_by_id_not_deleted(id, td)?;
        let users_on_team: Vec<User> = users::table
            .select(users::all_columns)
            .filter(users::team_id.eq(Some(id)))
            .load::<User>(td.get_db_connection())
            .map_err(|_| anyhow::Error::msg("Users for team could not be found!"))?;

        Ok((team, users_on_team))
    }
}

pub type ITeamRepo = Box<dyn TeamRepo>;

impl<'a, 'r> FromRequest<'a, 'r> for DbTeamRepo {
    type Error = ServiceError;

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<DbTeamRepo, Self::Error> {
        request::Outcome::Success(DbTeamRepo::new())
    }
}
