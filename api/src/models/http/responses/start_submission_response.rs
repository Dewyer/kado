use crate::models::submission::SubmissionDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct StartSubmissionResponse {
    pub submission: SubmissionDto,
}