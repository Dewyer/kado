
#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CreateTeamRequest {
    pub name: String,
}
