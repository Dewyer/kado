use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult};

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

    fn generate_submission_test_input(&self, _payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult> {
        Ok(SubmissionTestGenerationResult {
            input: serde_json::Value::Null,
            test_class: "".to_string(),
        })
    }
}
