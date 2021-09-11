use crate::db::transaction_manager::{TransactionManager, ITransaction};
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::models::team::{TeamFullyPopulatedDto, Team, NewTeam};
use crate::guards::{AuthTokenGuard, AccessToken};
use crate::models::http::responses::{GetUsersTeamResponse, CreateTeamResponse};
use crate::db::team_repo::{ITeamRepo, DbTeamRepo};
use crate::models::http::requests::CreateTeamRequest;
use crate::models::user::User;
use base64::CharacterSet::Crypt;
use crate::services::crypto_service::CryptoService;

pub struct TeamService {
    team_repo: ITeamRepo,
    tm: TransactionManager,
}

impl TeamService {
    pub fn new(
        team_repo: ITeamRepo,
        tm: TransactionManager,
    ) -> Self {
        Self {
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
            let user = user_guard.user;
            self.assert_user_can_create_team(&user, &payload, &td);

            let new_team = self.team_repo.crud().insert(&NewTeam {
                name: &payload.name,
                join_code: &self.get_new_join_code_for_team(),
                owner_user: Some(user.id),
                is_deleted: false,
            }, &td)?;

            Ok(CreateTeamResponse {
                team: TeamFullyPopulatedDto::from_team_and_users(&new_team, &vec![user], true),
            })
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TeamService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<TeamService, Self::Error> {
        let user_repo = req.guard::<DbTeamRepo>()?;
        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(TeamService::new(
            Box::new(user_repo),
            db_tm,
        ))
    }
}
