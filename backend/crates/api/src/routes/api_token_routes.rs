use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::api_result::AnyApiResult;
use crate::models::http::responses::GetApiTokenResponse;
use crate::services::api_token_service::ApiTokenService;
use rocket_okapi::openapi;

#[openapi]
#[get("/api-token")]
/// Get active token for logged in user
pub fn get_api_token(
    user_guard: AuthTokenGuard<AccessToken>,
    api_token_service: ApiTokenService,
) -> AnyApiResult<GetApiTokenResponse> {
    api_token_service.get_api_token_for_user(user_guard).into()
}

#[openapi]
#[post("/refresh-api-token")]
/// Refresh api token for logged in user
pub fn refresh_api_token(
    user_guard: AuthTokenGuard<AccessToken>,
    api_token_service: ApiTokenService,
) -> AnyApiResult<GetApiTokenResponse> {
    api_token_service
        .refresh_api_token_for_user(user_guard)
        .into()
}
