use crate::errors::AuthError;
use crate::models::user::User;
use crate::services::auth_service::AuthService;
use crate::services::utils_service::UtilsService;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};
use std::marker::PhantomData;
use crate::services::api_token_service::ApiTokenService;

pub const REFRESH_TOKEN_HEADER: &'static str = "X-Refresh-Token";
pub const API_TOKEN_HEADER: &'static str = "X-Api-Token";

pub trait AuthTokenDef {
    fn get_token_from_request(req: &Request<'_>) -> String;

    fn is_api_token() -> bool { false }
}

pub struct AuthTokenGuard<T: AuthTokenDef> {
    pub user: User,

    ph: PhantomData<T>,
}

pub struct AccessToken {}

impl AuthTokenDef for AccessToken {
    fn get_token_from_request(req: &Request<'_>) -> String {
        UtilsService::get_bearer_token_from_header(req)
    }
}

pub struct RefreshToken {}

impl AuthTokenDef for RefreshToken {
    fn get_token_from_request(req: &Request<'_>) -> String {
        UtilsService::header_value_or_empty_from_request(req, REFRESH_TOKEN_HEADER)
    }
}

pub struct ApiToken {}

impl AuthTokenDef for ApiToken {
    fn get_token_from_request(req: &Request<'_>) -> String {
        UtilsService::header_value_or_empty_from_request(req, API_TOKEN_HEADER)
    }

    fn is_api_token() -> bool {
        true
    }
}

impl<'a, 'r, T: AuthTokenDef> FromRequest<'a, 'r> for AuthTokenGuard<T> {
    type Error = AuthError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AuthTokenGuard<T>, Self::Error> {
        let auth_service = req
            .guard::<AuthService>()
            .map_failure(|_| (Status::Forbidden, AuthError::AuthTokenFailed))?;
        let api_token_service = req
            .guard::<ApiTokenService>()
            .map_failure(|_| (Status::Forbidden, AuthError::AuthTokenFailed))?;

        let token = T::get_token_from_request(req);

        let user_res = if T::is_api_token()
        { api_token_service.get_authenticated_user_by_api_token(token) } else { auth_service.get_authenticated_user_by_authorization_token(token) };

        if let Ok(user) = user_res {
            request::Outcome::Success(AuthTokenGuard {
                user,
                ph: PhantomData::default(),
            })
        } else {
            request::Outcome::Failure((Status::Forbidden, AuthError::AuthTokenFailed))
        }
    }
}
