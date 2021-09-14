#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CheckUserResponse {
    pub user_inactive: bool,
    pub user_exists: bool,
}