#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CheckUserRequest {
    pub authorizer: String,
    pub token: String,
}
