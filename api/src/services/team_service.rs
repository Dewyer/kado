use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request};
use crate::models::team::{TeamFullyPopulatedDto, Team};
use crate::guards::{AuthTokenGuard, AccessToken};
use crate::models::http::responses::GetUsersTeamResponse;
use crate::db::team_repo::{ITeamRepo, DbTeamRepo};

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
