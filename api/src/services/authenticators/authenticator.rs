use crate::services::authenticators::models::{
    AuthenticationResult,
    AuthenticationPayload,
};


pub trait Authenticator {
    fn authenticate(&self, payload: AuthenticationPayload) -> anyhow::Result<AuthenticationResult>;
}

pub type IAuthenticator = Box<dyn Authenticator>;