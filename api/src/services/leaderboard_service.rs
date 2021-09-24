use crate::db::team_repo::{DbTeamRepo, ITeamRepo};
use crate::db::transaction_manager::{ITransaction, TransactionManager};
use crate::db::user_repo::{DbUserRepo, IUserRepo, DEFAULT_USER_PAGE_SIZE};
use crate::errors::ServiceError;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::responses::{
    GetIndividualLeaderboardResponse, GetTeamLeaderboardResponse,
};
use crate::models::team::{Team, TeamLeaderboardEntryDto};
use crate::models::user::{User, UserLeaderboardEntryDto};
use crate::models::utils::PaginationOptions;
use rocket::request::FromRequest;
use rocket::{request, Request};

pub struct LeaderboardService {
    user_repo: IUserRepo,
    team_repo: ITeamRepo,
    tm: TransactionManager,
}

impl LeaderboardService {
    pub fn new(user_repo: IUserRepo, team_repo: ITeamRepo, tm: TransactionManager) -> Self {
        Self {
            user_repo,
            team_repo,
            tm,
        }
    }

    fn get_team_for_user(&self, user: &User, td: &ITransaction) -> anyhow::Result<Option<Team>> {
        if let Some(team_id) = user.team_id {
            let team = self.team_repo.crud().find_by_id(team_id, td)?;
            Ok(Some(team))
        } else {
            Ok(None)
        }
    }

    fn get_current_user_leaderboard_entry(
        &self,
        user: &User,
        td: &ITransaction,
    ) -> anyhow::Result<UserLeaderboardEntryDto> {
        let user_team = self.get_team_for_user(&user, td)?;
        let user_rank = self.user_repo.find_leaderboard_rank_of_user(&user, &td)?;

        Ok(user.to_leaderboard_dto(user_rank, user_team, false))
    }

    fn get_current_user_team_leaderboard_entry(
        &self,
        user: &User,
        td: &ITransaction,
    ) -> anyhow::Result<Option<TeamLeaderboardEntryDto>> {
        if let Some(team_id) = user.team_id {
            let team = self.team_repo.crud().find_by_id(team_id, td)?;
            let team_rank = self.team_repo.find_leaderboard_rank_of_team(&team, td)?;

            Ok(Some(team.to_leaderboard_dto(team_rank, false)))
        } else {
            Ok(None)
        }
    }

    fn get_individual_leaderboard_paginated(
        &self,
        pagination_options: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<UserLeaderboardEntryDto>, usize)> {
        let (users_with_teams, page_count) = self
            .user_repo
            .list_leaderboard_participating_paginated_with_teams(&pagination_options, &td)?;
        let first_user_rank = pagination_options
            .per_page
            .unwrap_or(DEFAULT_USER_PAGE_SIZE)
            * pagination_options.page;
        let leaderboard = users_with_teams
            .into_iter()
            .enumerate()
            .map(|(ii, user_with_team)| {
                user_with_team.0.to_leaderboard_dto(
                    ii + first_user_rank,
                    user_with_team.1,
                    !user_with_team.0.participate_in_leaderboards,
                )
            })
            .collect::<Vec<UserLeaderboardEntryDto>>();

        Ok((leaderboard, page_count))
    }

    fn get_team_leaderboard_paginated(
        &self,
        pagination_options: &PaginationOptions,
        td: &ITransaction,
    ) -> anyhow::Result<(Vec<TeamLeaderboardEntryDto>, usize)> {
        let (teams, page_count) = self
            .team_repo
            .list_leaderboard_participating_paginated(&pagination_options, &td)?;
        let first_team_rank = pagination_options
            .per_page
            .unwrap_or(DEFAULT_USER_PAGE_SIZE)
            * pagination_options.page;
        let leaderboard = teams
            .into_iter()
            .enumerate()
            .map(|(ii, team)| {
                team.to_leaderboard_dto(ii + first_team_rank, !team.participate_in_leaderboards)
            })
            .collect::<Vec<TeamLeaderboardEntryDto>>();

        Ok((leaderboard, page_count))
    }

    pub fn get_individual_leaderboard(
        &self,
        user_guard: AuthTokenGuard<AccessToken>,
        pagination_options: PaginationOptions,
    ) -> anyhow::Result<GetIndividualLeaderboardResponse> {
        self.tm.transaction(|td| {
            let user = user_guard.user;
            let (leaderboard, page_count) =
                self.get_individual_leaderboard_paginated(&pagination_options, &td)?;

            Ok(GetIndividualLeaderboardResponse {
                leaderboard,
                page_count,
                user_ranking: self.get_current_user_leaderboard_entry(&user, &td)?,
            })
        })
    }

    pub fn get_team_leaderboard(
        &self,
        user_guard: AuthTokenGuard<AccessToken>,
        pagination_options: PaginationOptions,
    ) -> anyhow::Result<GetTeamLeaderboardResponse> {
        self.tm.transaction(|td| {
            let user = user_guard.user;
            let (leaderboard, page_count) =
                self.get_team_leaderboard_paginated(&pagination_options, &td)?;

            Ok(GetTeamLeaderboardResponse {
                leaderboard,
                page_count,
                user_team_ranking: self.get_current_user_team_leaderboard_entry(&user, &td)?,
            })
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
