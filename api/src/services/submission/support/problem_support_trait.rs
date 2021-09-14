
pub struct SubmissionGenerationPayload {
    pub seed: i32,
}

pub struct SubmissionGenerationResult {
    pub test_count: i32,
}

pub struct SubmissionTestGenerationPayload {
    pub seed: i32,
    pub test_index: usize,
}

pub struct SubmissionTestGenerationResult {
    pub input: serde_json::Value,
    pub test_class: String,
}

pub trait ProblemSupport {
    fn generate_submission_details(&self, payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult>;

    fn generate_submission_test_input(&self, payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult>;
}

pub type IProblemSupport = Box<dyn ProblemSupport>;