use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult};

pub struct CamelCaseProblemSupport {
}

impl CamelCaseProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProblemSupport for CamelCaseProblemSupport {
    fn generate_submission_details(&self, _payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: 3,
        })
    }
}
