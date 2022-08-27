use crate::models::problem::ProblemFullyPopulatedDto;
use crate::models::submission::SubmissionDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetProblemDetailsResponse {
    pub problem: ProblemFullyPopulatedDto,
    pub submissions: Vec<SubmissionDto>,
}
