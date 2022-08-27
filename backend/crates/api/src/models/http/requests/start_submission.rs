#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct StartSubmissionRequest {
    pub problem: String,
    pub sample_index: Option<i32>,
}
