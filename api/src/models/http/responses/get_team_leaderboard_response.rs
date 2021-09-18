use crate::models::team::TeamLeaderboardEntryDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTeamLeaderboardResponse {
    pub leaderboard: Vec<TeamLeaderboardEntryDto>,
    pub page_count: usize,
    pub user_team_ranking: Option<TeamLeaderboardEntryDto>,
}
