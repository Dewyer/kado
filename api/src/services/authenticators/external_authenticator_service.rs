use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use rocket::http::Status;
use crate::services::authenticators::google_authenticator_service::GoogleAuthenticatorService;
use crate::services::authenticators::models::{AuthenticationPayload, AuthenticationResult};
use crate::services::authenticators::models::Authorizer;
use crate::services::authenticators::authenticator::{Authenticator, IAuthenticator};
use google_jwt_verify::Client;

pub struct ExternalAuthenticatorService {
    google_authenticator_service: IAuthenticator,
}

impl ExternalAuthenticatorService {
    pub fn new(google_authenticator_service: IAuthenticator) -> Self {
        Self {
            google_authenticator_service,
        }
    }

    pub fn authenticate(&self, payload: AuthenticationPayload) -> anyhow::Result<AuthenticationResult> {
        match payload.authorizer {
            Authorizer::Google => self.google_authenticator_service.authenticate(payload),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ExternalAuthenticatorService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<ExternalAuthenticatorService, Self::Error> {
        let google_authenticator_service = req.guard::<GoogleAuthenticatorService>()?;

        request::Outcome::Success(ExternalAuthenticatorService::new(
            Box::new(google_authenticator_service),
        ))
    }
}
