use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::api_result::AnyApiResult;
use crate::models::http::responses::{
    GetIndividualLeaderboardResponse, GetTeamLeaderboardResponse,
};
use crate::models::utils::PaginationOptions;
use crate::services::leaderboard_service::LeaderboardService;
use rocket_okapi::openapi;

#[openapi]
#[get("/leaderboard/individual?<per_page>&<page>")]
/// Get individual leaderboard
pub fn get_individual_leaderboard(
    user_guard: AuthTokenGuard<AccessToken>,
    per_page: Option<usize>,
    page: usize,
    leaderboard_service: LeaderboardService,
) -> AnyApiResult<GetIndividualLeaderboardResponse> {
    leaderboard_service
        .get_individual_leaderboard(user_guard, PaginationOptions { per_page, page })
        .into()
}

#[openapi]
#[get("/leaderboard/team?<per_page>&<page>")]
/// Get team leaderboard
pub fn get_team_leaderboard(
    user_guard: AuthTokenGuard<AccessToken>,
    per_page: Option<usize>,
    page: usize,
    leaderboard_service: LeaderboardService,
) -> AnyApiResult<GetTeamLeaderboardResponse> {
    leaderboard_service
        .get_team_leaderboard(user_guard, PaginationOptions { per_page, page })
        .into()
}
