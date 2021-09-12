use crate::models::problem::ProblemFullyPopulatedDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetProblemDetailsResponse {
    pub problem: ProblemFullyPopulatedDto,
}