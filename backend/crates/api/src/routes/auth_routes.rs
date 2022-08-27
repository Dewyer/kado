use crate::guards::{AuthTokenGuard, RefreshToken};
use crate::models::http::api_result::AnyApiResult;
use crate::models::http::requests::{
    CheckUserRequest, ExchangeGithubCodeRequest, LoginRequest, RegisterRequest,
};
use crate::models::http::responses::{
    AuthorizingResponse, CheckUserResponse, ExchangeGithubCodeResponse,
};
use crate::services::auth_service::AuthService;
use crate::services::authenticators::github_authenticator_service::GithubAuthenticatorService;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[post("/auth/check-user", format = "json", data = "<request>")]
/// Check if a user is already registered by a token
pub fn check_user(
    request: Json<CheckUserRequest>,
    auth_service: AuthService,
) -> AnyApiResult<CheckUserResponse> {
    auth_service.check_user(request.0).into()
}

#[openapi]
#[post("/auth/register", format = "json", data = "<request>")]
/// Register a new user
pub fn register_user(
    request: Json<RegisterRequest>,
    auth_service: AuthService,
) -> AnyApiResult<AuthorizingResponse> {
    auth_service.register_user(request.0).into()
}

#[openapi]
#[post("/auth/login", format = "json", data = "<request>")]
/// Login an existing user
pub fn login_user(
    request: Json<LoginRequest>,
    auth_service: AuthService,
) -> AnyApiResult<AuthorizingResponse> {
    auth_service.login_user(request.0).into()
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

#[openapi]
#[post("/auth/exchange-github-code", format = "json", data = "<request>")]
/// Exchange github access code for token
pub fn exchange_github_code(
    request: Json<ExchangeGithubCodeRequest>,
    github_auth_service: GithubAuthenticatorService,
) -> AnyApiResult<ExchangeGithubCodeResponse> {
    github_auth_service
        .exchange_code_for_token(&request.0.code)
        .map(|token| ExchangeGithubCodeResponse { token })
        .into()
}
