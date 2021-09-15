use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::responses::{GetProblemsResponse, GetProblemDetailsResponse};
use crate::services::leaderboard_service::LeaderboardService;
use crate::models::utils::PaginationOptions;

#[openapi]
#[get("/leaderboard/individual?<per_page>&<page>")]
/// Get individual leaderboard
pub fn get_individual_leaderboard(
    user_guard: AuthTokenGuard<AccessToken>,
    per_page: Option<usize>,
    page: usize,
    leaderboard_service: LeaderboardService,
) -> AnyApiResult<GetProblemsResponse> {
    leaderboard_service
        .get_individual_leaderboard(user_guard, PaginationOptions {
            per_page,
            page,
        })
        .into()
}

/*
#[openapi]
#[get("/leaderboard/team?<per_page>&<page>")]
/// Get team leaderboard
pub fn get_team_leaderboards(
    user_guard: AuthTokenGuard<AccessToken>,
    per_page: usize,
    page: usize,
    leaderboard_service: LeaderboardService,
) -> AnyApiResult<GetProblemDetailsResponse> {
    problem_service
        .get_problem_details(user_guard, code_name)
        .into()
}
*/