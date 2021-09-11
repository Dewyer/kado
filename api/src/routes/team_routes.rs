use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::responses::GetUsersTeamResponse;
use crate::services::team_service::TeamService;

#[openapi]
#[get("/team")]
/// Get the team of the logged in user
pub fn get_authenticated_users_team(
    user_guard: AuthTokenGuard<AccessToken>,
    team_service: TeamService,
) -> AnyApiResult<GetUsersTeamResponse> {
    team_service
        .get_full_team_for_user(user_guard)
        .into()
}