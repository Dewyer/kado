use crate::models::submission::submission_test::SubmissionTest;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubmissionGenerationPayload {
    pub seed: i64,
    pub sample_index: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubmissionGenerationResult {
    pub test_count: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubmissionTestGenerationPayload {
    pub seed: i64,
    pub test_index: usize,
    pub sample_index: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubmissionTestGenerationResult {
    pub input: serde_json::Value,
    pub test_class: String,
}

pub struct VerificationPayload<'a> {
    pub test: &'a SubmissionTest,
    pub output: &'a serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerificationPayloadOwned{
    pub test: SubmissionTest,
    pub output: serde_json::Value,
}

impl VerificationPayloadOwned {
    pub fn from_ref(payload: &VerificationPayload) -> Self {
        Self {
            test: payload.test.clone(),
            output: payload.output.clone(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerificationResult {
    pub correct: bool,
}

pub trait ProblemSupport {
    fn generate_submission_details(
        &self,
        payload: SubmissionGenerationPayload,
    ) -> anyhow::Result<SubmissionGenerationResult>;

    fn generate_submission_test_input(
        &self,
        payload: SubmissionTestGenerationPayload,
    ) -> anyhow::Result<SubmissionTestGenerationResult>;

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult>;
}

pub type IProblemSupport = Box<dyn ProblemSupport>;
