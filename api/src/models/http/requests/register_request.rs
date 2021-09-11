
#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct RegisterRequest {
    pub authorizer: String,
    pub token: String,

    pub username: String,
    pub participate_in_leaderboards: bool,
}
