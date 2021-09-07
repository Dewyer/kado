
#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CheckUserRequest {
    pub authenticator: String,
    pub token: String,
}
