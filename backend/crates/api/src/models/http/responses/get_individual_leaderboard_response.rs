use crate::models::user::UserLeaderboardEntryDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetIndividualLeaderboardResponse {
    pub leaderboard: Vec<UserLeaderboardEntryDto>,
    pub page_count: usize,
    pub user_ranking: UserLeaderboardEntryDto,
}
