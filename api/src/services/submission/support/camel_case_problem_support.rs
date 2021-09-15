use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload, VerificationResult};

pub struct CamelCaseProblemSupport {}

impl CamelCaseProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize)]
struct CamelCaseInput {
    word: String,
}

#[derive(serde::Deserialize)]
struct CamelCaseOutput {
    word_count: i32,
}

impl ProblemSupport for CamelCaseProblemSupport {
    fn generate_submission_details(&self, _payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: 1,
        })
    }

    fn generate_submission_test_input(&self, _payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult> {
        let input = serde_json::to_value(CamelCaseInput {
            word: "camelCaseIsLol".to_string(),
        }).map_err(|_| anyhow::Error::msg("Invalid input!"))?;

        Ok(SubmissionTestGenerationResult {
            input,
            test_class: "".to_string(),
        })
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        let output = serde_json::from_value::<CamelCaseOutput>(payload.output.clone())?;

        Ok(VerificationResult {
            correct: output.word_count == 4,
        })
    }
}
