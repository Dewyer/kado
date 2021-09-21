#[cfg(test)]
mod tests {
    use crate::services::submission::support::{SanityCheckProblemSupport, ProblemSupport, SubmissionGenerationPayload, SubmissionTestGenerationPayload, SanityCheckInput, VerificationPayload};
    use crate::models::submission::submission_test::SubmissionTest;
    use crate::services::utils_service::UtilsService;

    fn get_support() -> SanityCheckProblemSupport {
        SanityCheckProblemSupport {}
    }

    #[test]
    fn sanity_check_generates_details() {
        let support = get_support();
        let details = support.generate_submission_details(SubmissionGenerationPayload {
            seed: 10,
            sample_index: None,
        }).unwrap();

        assert_eq!(details.test_count, 4);
    }

    #[test]
    fn sanity_check_generates_input() {
        let support = get_support();
        let details = support.generate_submission_test_input(SubmissionTestGenerationPayload {
            seed: 10,
            test_index: 3,
            sample_index: None,
        }).unwrap();

        let inp = serde_json::from_value::<SanityCheckInput>(details.input)
            .unwrap();

        println!("{:?}", inp);
        assert_eq!(inp.set.len(), inp.meta.set_length);
        assert_ne!(inp.set.len(), 0);
    }

    #[test]
    fn sanity_check_generates_output() {
        let support = get_support();
        let details = support.generate_submission_test_input(SubmissionTestGenerationPayload {
            seed: 10,
            test_index: 3,
            sample_index: None,
        }).unwrap();
        let inp = serde_json::from_value::<SanityCheckInput>(details.input.clone())
            .unwrap();

        let output = SanityCheckProblemSupport::solver(&details.input.to_string()).unwrap();
        println!("{:?}", inp);
        println!("{:?}", output);
    }

    #[test]
    fn sanity_check_generates_output_conformity() {
        let support = get_support();
        let details = support.generate_submission_test_input(SubmissionTestGenerationPayload {
            seed: 10,
            test_index: 3,
            sample_index: None,
        }).unwrap();
        let inp = serde_json::from_value::<SanityCheckInput>(details.input.clone())
            .unwrap();

        let output = SanityCheckProblemSupport::solver(&details.input.to_string()).unwrap();
        let verf = support.verify_output(VerificationPayload {
            output: &serde_json::to_value(output).unwrap(),
            test: &SubmissionTest {
                id: Default::default(),
                submission_id: Default::default(),
                class: "".to_string(),
                input: serde_json::to_string(&inp).unwrap(),
                output: None,
                correct: None,
                started_at: UtilsService::naive_now(),
                finished_at: None
            },
        }).unwrap();

        assert_eq!(verf.correct, true)
    }
}