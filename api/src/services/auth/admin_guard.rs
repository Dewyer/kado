use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};

use crate::errors::AuthError;
use crate::services::auth::AuthService;

pub struct AdminGuard {}

const ADMIN_KEY_HEADER: &'static str = "Authorization";

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = AuthError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AdminGuard, Self::Error> {
        let auth_service = req
            .guard::<AuthService>()
            .map_failure(|_| (Status::Forbidden, AuthError::AdminAuthError))?;

        let admin_key_header = req.headers().get(ADMIN_KEY_HEADER).next().unwrap_or("");
        if auth_service.is_admin_key_correct(admin_key_header) {
            request::Outcome::Success(AdminGuard {})
        } else {
            request::Outcome::Failure((Status::Forbidden, AuthError::AdminAuthError))
        }
    }
}
