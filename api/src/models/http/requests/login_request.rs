#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct LoginRequest {
    pub authorizer: String,
    pub token: String,
}
