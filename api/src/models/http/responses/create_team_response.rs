use crate::models::team::TeamFullyPopulatedDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct CreateTeamResponse {
    pub team: TeamFullyPopulatedDto,
}