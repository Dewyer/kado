
pub struct SubmissionGenerationPayload {
    pub seed: i32,
}

pub struct SubmissionGenerationResult {
    pub test_count: i32,
}

pub trait ProblemSupport {
    fn generate_submission_details(&self, payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult>;
}

pub type IProblemSupport = Box<dyn ProblemSupport>;