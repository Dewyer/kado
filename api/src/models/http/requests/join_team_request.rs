#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct JoinTeamRequest {
    pub join_code: String,
}
