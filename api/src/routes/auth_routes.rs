use rocket_contrib::json::Json;
use rocket_okapi::openapi;
use crate::guards::{AuthTokenGuard, RefreshToken};
use crate::models::http::api_result::{AnyApiResult, ApiResult, ErrorResponse};
use crate::services::auth_service::AuthService;
use crate::models::http::responses::{AuthorizingResponse, CheckUserResponse};
use crate::models::http::requests::CheckUserRequest;


#[openapi]
#[post("/auth/check-user", format = "json", data = "<request>")]
/// Check if a user is already registered by a token
pub fn check_user(
    request: Json<CheckUserRequest>,
    auth_service: AuthService,
) -> AnyApiResult<CheckUserResponse> {
    auth_service
        .authenticate_user_by_refresh_token(refresh_token_guard)
        .into()
}

#[openapi]
#[post("/auth/refresh")]
/// Refresh user session with refresh token
pub fn refresh_token(
    refresh_token_guard: AuthTokenGuard<RefreshToken>,
    auth_service: AuthService,
) -> AnyApiResult<AuthorizingResponse> {
    auth_service
        .authenticate_user_by_refresh_token(refresh_token_guard)
        .into()
}
