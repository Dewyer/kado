use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use rocket::http::Status;

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
