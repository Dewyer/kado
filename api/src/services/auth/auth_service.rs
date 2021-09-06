use crate::config::AppConfig;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::http::Status;
use rocket::{Request, request, State};

pub struct AuthService {
    app_config: AppConfig,
}

impl AuthService {
    pub fn new(app_config: AppConfig) -> Self {
        Self {
            app_config,
        }
    }

    pub fn is_admin_key_correct(&self, admin_key: &str) -> bool {
        self.app_config.admin_key == admin_key
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<AuthService, Self::Error> {
        let config = req.guard::<State<AppConfig>>().map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })?;

        request::Outcome::Success(AuthService::new(
            config.inner().clone(),
        ))
    }
}
