use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use rocket::http::Status;
use crate::services::authenticators::models::{AuthenticationResult, AuthenticationPayload};
use crate::services::authenticators::authenticator::Authenticator;
use google_jwt_verify::Client;

pub struct GoogleAuthenticatorService {
    config: AppConfig,
}

impl GoogleAuthenticatorService {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
        }
    }
}

impl Authenticator for GoogleAuthenticatorService {
    fn authenticate(&self, payload: AuthenticationPayload) -> anyhow::Result<AuthenticationResult> {
        let client = Client::new(&self.config.google_client_id);
        let id_token = client.verify_id_token(&payload.token)
            .map_err(|_| anyhow::Error::msg("Invalid google token!"))?;

        let email = id_token.get_payload().get_email();

        Ok(
            AuthenticationResult {
                email,
            }
        )
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for GoogleAuthenticatorService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<GoogleAuthenticatorService, Self::Error> {
        let config = req.guard::<State<AppConfig>>().map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })?;

        request::Outcome::Success(GoogleAuthenticatorService::new(
            config.inner().clone(),
        ))
    }
}
