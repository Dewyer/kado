use crate::models::submission::SubmissionMinimalDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct StartSubmissionResponse {
    pub submission: SubmissionMinimalDto,
}
