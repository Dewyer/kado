use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload, VerificationResult};

pub struct SanityCheckProblemSupport {}

impl SanityCheckProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SanityCheckInputMeta {
    set_length: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SanityCheckInput {
    set: Vec<usize>,
    meta: SanityCheckInputMeta,
}

#[derive(serde::Deserialize)]
struct SanityCheckOutput {
    insane_numbers: Vec<usize>,
}

impl ProblemSupport for SanityCheckProblemSupport {
    fn generate_submission_details(&self, _payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: 1,
        })
    }

    fn generate_submission_test_input(&self, _payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult> {
        todo!();
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        todo!();
    }
}
