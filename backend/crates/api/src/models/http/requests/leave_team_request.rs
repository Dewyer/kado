#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct LeaveTeamRequest {
    pub inheritor: Option<String>,
}
