#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub participate_in_leaderboards: bool,
    pub is_admin: bool,
    pub team_id: Option<String>,
}