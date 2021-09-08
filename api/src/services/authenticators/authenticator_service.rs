use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use rocket::http::Status;
use crate::services::authenticators::google_authenticator_service::GoogleAuthenticatorService;

pub struct AuthenticatorService {
    google_authenticator_service: GoogleAuthenticatorService,
}

impl AuthenticatorService {
    pub fn new(google_authenticator_service: GoogleAuthenticatorService) -> Self {
        Self {
            google_authenticator_service,
        }
    }


}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatorService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AuthenticatorService, Self::Error> {
        let google_authenticator_service = req.guard::<GoogleAuthenticatorService>()?;

        request::Outcome::Success(AuthenticatorService::new(
            google_authenticator_service,
        ))
    }
}
