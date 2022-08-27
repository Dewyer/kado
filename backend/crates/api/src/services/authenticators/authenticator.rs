use crate::services::authenticators::models::{AuthenticationPayload, AuthenticationResult};

pub trait Authenticator {
    fn authenticate(&self, payload: AuthenticationPayload) -> anyhow::Result<AuthenticationResult>;
}

pub type IAuthenticator = Box<dyn Authenticator>;
