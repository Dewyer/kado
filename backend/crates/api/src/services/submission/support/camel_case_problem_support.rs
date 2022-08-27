use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload, VerificationResult};

pub struct CamelCaseProblemSupport {}

impl CamelCaseProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CamelCaseInput {
    word: String,
}

#[derive(serde::Deserialize)]
struct CamelCaseOutput {
    word_count: i32,
}

impl CamelCaseProblemSupport {
    fn get_input_for(&self, val: &str) -> anyhow::Result<SubmissionTestGenerationResult> {
        let input = serde_json::to_value(CamelCaseInput {
            word: val.to_string(),
        }).map_err(|_| anyhow::Error::msg("Invalid input!"))?;

        Ok(SubmissionTestGenerationResult {
            input,
            test_class: "".to_string(),
        })
    }
}

impl ProblemSupport for CamelCaseProblemSupport {
    fn generate_submission_details(&self, _payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: 1,
        })
    }

    fn generate_submission_test_input(&self, payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult> {
        if payload.sample_index.is_some() {
            self.get_input_for("camelCaseIsLol")
        } else {
            self.get_input_for("helloWorld")
        }
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        let input = serde_json::from_str::<CamelCaseInput>(&payload.test.input)?;
        let output = serde_json::from_value::<CamelCaseOutput>(payload.output.clone())?;

        Ok(VerificationResult {
            correct: if input.word == "helloWorld" { output.word_count == 2 } else { output.word_count == 4 },
        })
    }
}
