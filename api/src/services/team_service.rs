use crate::db::transaction_manager::{TransactionManager, ITransaction};
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::models::team::{TeamFullyPopulatedDto, Team, NewTeam};
use crate::guards::{AuthTokenGuard, AccessToken};
use crate::models::http::responses::{GetUsersTeamResponse, CreateTeamResponse};
use crate::db::team_repo::{ITeamRepo, DbTeamRepo};
use crate::models::http::requests::{CreateTeamRequest, LeaveTeamRequest, JoinTeamRequest};
use crate::models::user::User;
use base64::CharacterSet::Crypt;
use crate::services::crypto_service::CryptoService;
use crate::db::user_repo::{IUserRepo, DbUserRepo};
use crate::services::utils_service::UtilsService;

pub struct TeamService {
    user_repo: IUserRepo,
    team_repo: ITeamRepo,
    tm: TransactionManager,
}

impl TeamService {
    pub fn new(
        user_repo: IUserRepo,
        team_repo: ITeamRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
            user_repo,
            team_repo,
            tm,
        }
    }

    pub fn get_full_team_for_user(
        &self,
        user_guard: AuthTokenGuard<AccessToken>,
    ) -> anyhow::Result<GetUsersTeamResponse> {
        self.tm.transaction(|td| {
            let user = user_guard.user;

            if let Some(team_id) = user.team_id.clone() {
                let team_opt = self.team_repo.find_by_id_with_users(team_id, &td)
                    .ok();
                if let Some(team) = team_opt {
                    let is_logged_in_user_owner = team.0.owner_user.contains(&user.id);
                    Ok(GetUsersTeamResponse {
                        team: Some(TeamFullyPopulatedDto::from_team_and_users(&team.0, &team.1, is_logged_in_user_owner)),
                    })
                } else {
                    Ok(GetUsersTeamResponse {
                        team: None,
                    })
                }

            } else {
                Ok(GetUsersTeamResponse {
                    team: None,
                })
            }
        })
    }

    fn get_new_join_code_for_team(&self) -> String {
        CryptoService::get_random_string(8)
    }

    pub fn assert_user_can_create_team(&self, user: &User, payload: &CreateTeamRequest, td: &ITransaction) -> anyhow::Result<()> {
        if let Some(user_team_id) = user.team_id.clone() {
            let team_res = self.team_repo.find_by_id_not_deleted(user_team_id, &td);
            if team_res.is_ok()
            {
                bail!("Can't create a new team while already in one!");
            }
        }

        let existing_team = self.team_repo.find_by_name(&payload.name, &td);
        if existing_team.is_ok() {
            bail!("Team with the same name already exists!");
        }

        Ok(())
    }

    pub fn create_team(&self, user_guard: AuthTokenGuard<AccessToken>, payload: CreateTeamRequest) -> anyhow::Result<CreateTeamResponse> {
        self.tm.transaction(|td| {
            let mut user = user_guard.user;
            self.assert_user_can_create_team(&user, &payload, &td);

            let new_team = self.team_repo.crud().insert(&NewTeam {
                name: &payload.name,
                join_code: &self.get_new_join_code_for_team(),
                owner_user: Some(user.id),
                is_deleted: false,
            }, &td)?;

            user.team_id = Some(new_team.id);
            self.user_repo.save(&user, &td)?;

            Ok(CreateTeamResponse {
                team: TeamFullyPopulatedDto::from_team_and_users(&new_team, &vec![user], true),
            })
        })
    }

    fn assert_user_can_leave_team(&self, user: &User) -> anyhow::Result<()> {
        if user.team_id.is_none() {
            bail!("User is not a team!");
        }

        Ok(())
    }

    fn handle_ownership_transfer_on_team_leave(
        &self,
        team_and_users: &mut (Team, Vec<User>),
        leave_payload: &LeaveTeamRequest,
        td: &ITransaction,
    ) -> anyhow::Result<()> {
        let (team, team_users) = team_and_users;
        if team_users.len() == 1 {
            team.is_deleted = true;
            team.owner_user = None;

            self.team_repo.save(&team, &td)?;
            return Ok(());
        }

        let inheritor_uuid = UtilsService::parse_optional_uuid(&leave_payload.inheritor)?
            .ok_or(anyhow::Error::msg("Inheritor required!"))?;

        if !team_users.iter().any(|el| el.id == inheritor_uuid) {
            bail!("Inheritor is not in the team!");
        }
        self.user_repo.crud().find_by_id(inheritor_uuid, &td).map_err(|_| anyhow::Error::msg("Inheritor not found!"))?;

        team.owner_user = Some(inheritor_uuid);
        self.team_repo.save(&team, &td)?;

        Ok(())
    }

    pub fn leave_team(&self, user_guard: AuthTokenGuard<AccessToken>, payload: LeaveTeamRequest) -> anyhow::Result<()> {
        self.tm.transaction(|td| {
            let mut user = user_guard.user;
            self.assert_user_can_leave_team(&user)?;
            let mut team_and_users = self.team_repo.find_by_id_with_users(user.team_id.unwrap(), &td)?;

            user.team_id = None;
            self.user_repo.save(&user, &td)?;

            if team_and_users.0.owner_user.contains(&user.id) {
                self.handle_ownership_transfer_on_team_leave(&mut team_and_users, &payload, &td)?;
            }

            Ok(())
        })
    }

    fn assert_user_can_join_team(&self, user: &User) -> anyhow::Result<()> {
        if user.team_id.is_some() {
            bail!("User is already in a team!");
        }

        Ok(())
    }

    pub fn join_team(&self, user_guard: AuthTokenGuard<AccessToken>, payload: JoinTeamRequest) -> anyhow::Result<()> {
        self.tm.transaction(|td| {
            let mut user = user_guard.user;
            self.assert_user_can_join_team(&user)?;

            let mut team_to_join = self.team_repo.find_by_join_code(&payload.join_code, &td)?;
            user.team_id = Some(team_to_join.id);
            if team_to_join.owner_user.is_none() {
                team_to_join.owner_user = Some(user.id);
                self.team_repo.save(&team_to_join, &td)?;
            }

            self.user_repo.save(&user, &td)?;
            Ok(())
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TeamService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<TeamService, Self::Error> {
        let user_repo = req.guard::<DbUserRepo>()?;
        let team_repo = req.guard::<DbTeamRepo>()?;

        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(TeamService::new(
            Box::new(user_repo),
            Box::new(team_repo),
            db_tm,
        ))
    }
}
