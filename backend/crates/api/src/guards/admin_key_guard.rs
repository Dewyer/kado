use crate::errors::AuthError;
use crate::services::auth_service::AuthService;
use crate::services::utils_service::UtilsService;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};

pub const ADMIN_KEY_HEADER: &'static str = "X-Admin-Key";

pub struct AdminKeyGuard {}

impl<'a, 'r> FromRequest<'a, 'r> for AdminKeyGuard {
    type Error = AuthError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AdminKeyGuard, Self::Error> {
        let auth_service = req
            .guard::<AuthService>()
            .map_failure(|_| (Status::Forbidden, AuthError::AuthTokenFailed))?;

        let token = UtilsService::header_value_or_empty_from_request(req, ADMIN_KEY_HEADER);

        if auth_service.is_admin_key_correct(token) {
            request::Outcome::Success(AdminKeyGuard {})
        } else {
            request::Outcome::Failure((Status::Forbidden, AuthError::AuthTokenFailed))
        }
    }
}
