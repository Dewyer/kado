use crate::db::user_repo::{IUserRepo, DbUserRepo};
use crate::db::transaction_manager::TransactionManager;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{request, Request};
use crate::db::team_repo::{ITeamRepo, DbTeamRepo};
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::responses::GetIndividualLeaderboardResponse;
use crate::models::utils::PaginationOptions;

pub struct LeaderboardService {
    user_repo: IUserRepo,
    team_repo: ITeamRepo,
    tm: TransactionManager,
}

impl LeaderboardService {
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

    pub fn get_individual_leaderboard(&self, user_guard: AuthTokenGuard<AccessToken>, pagination_options: PaginationOptions) -> anyhow::Result<GetIndividualLeaderboardResponse> {
        self.tm.transaction(|td| {
            
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for LeaderboardService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<LeaderboardService, Self::Error> {
        let user_repo = req.guard::<DbUserRepo>()?;
        let team_repo = req.guard::<DbTeamRepo>()?;

        let db_tm = req.guard::<TransactionManager>()?;

        request::Outcome::Success(LeaderboardService::new(
            Box::new(user_repo),
            Box::new(team_repo),
            db_tm,
        ))
    }
}
