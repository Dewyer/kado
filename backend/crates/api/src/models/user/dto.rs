#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub participate_in_leaderboards: bool,
    pub individual_points: i64,
    pub last_gained_points_at: Option<String>,
    pub is_admin: bool,
    pub team_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct UserLeaderboardEntryDto {
    pub id: String,
    pub rank: usize,
    pub username: String,
    pub individual_points: i64,
    pub team_name: Option<String>,
}
