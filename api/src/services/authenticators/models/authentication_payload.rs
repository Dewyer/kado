use crate::services::authenticators::models::Authorizer;

pub struct AuthenticationPayload {
    pub authorizer: Authorizer,
    pub token: String,
}
