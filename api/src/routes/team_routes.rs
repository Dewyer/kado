use rocket_contrib::json::Json;
use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::responses::{GetUsersTeamResponse, CreateTeamResponse};
use crate::services::team_service::TeamService;
use crate::models::http::requests::{CreateTeamRequest, LeaveTeamRequest, JoinTeamRequest};

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

#[openapi]
#[post("/team/create", format = "json", data = "<request>")]
/// Create a new team
pub fn create_team(
    user_guard: AuthTokenGuard<AccessToken>,
    request: Json<CreateTeamRequest>,
    team_service: TeamService,
) -> AnyApiResult<CreateTeamResponse> {
    team_service
        .create_team(user_guard, request.0)
        .into()
}

#[openapi]
#[post("/team/join", format = "json", data = "<request>")]
/// Join a team
pub fn join_team(
    user_guard: AuthTokenGuard<AccessToken>,
    request: Json<JoinTeamRequest>,
    team_service: TeamService,
) -> AnyApiResult<()> {
    team_service
        .join_team(user_guard, request.0)
        .into()
}

#[openapi]
#[post("/team/leave", format = "json", data = "<request>")]
/// Leave team user is in
pub fn leave_team(
    user_guard: AuthTokenGuard<AccessToken>,
    request: Json<LeaveTeamRequest>,
    team_service: TeamService,
) -> AnyApiResult<()> {
    team_service
        .leave_team(user_guard, request.0)
        .into()
}