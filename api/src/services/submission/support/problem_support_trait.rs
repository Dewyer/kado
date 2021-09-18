use crate::models::submission::submission_test::SubmissionTest;

pub struct SubmissionGenerationPayload {
    pub seed: i32,
    pub sample_index: Option<i32>,
}

pub struct SubmissionGenerationResult {
    pub test_count: i32,
}

pub struct SubmissionTestGenerationPayload {
    pub seed: i32,
    pub test_index: usize,
    pub sample_index: Option<i32>,
}

pub struct SubmissionTestGenerationResult {
    pub input: serde_json::Value,
    pub test_class: String,
}

pub struct VerificationPayload<'a> {
    pub test: &'a SubmissionTest,
    pub output: &'a serde_json::Value,
}

pub struct VerificationResult {
    pub correct: bool,
}

pub trait ProblemSupport {
    fn generate_submission_details(&self, payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult>;

    fn generate_submission_test_input(&self, payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult>;

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult>;
}

pub type IProblemSupport = Box<dyn ProblemSupport>;
