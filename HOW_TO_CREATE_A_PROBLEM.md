### How to create a problem for this site?

- Steps:
  - Read the API guide so you own how the submission process looks like for competitors
  - Create a migration for your problem in /api/migrations examples can be seen there
  - Create a problem statement for it in /admin/statements/<code_name>.md
  - Create a problem support program

#### Problem support program
- Determines test count and samples
- Generates test inputs
- Verifies test outputs

Can be a rust service in the backend or an external microservice.

#### Problem support microservice interface

- Authorization: X-Api-Key header, and we will agree on a long string value for this

##### Types
- Uuid - String UUID
- Option<T> - Some type that is nullable
- i64 - 64 bit integer
- i32 - 32 bit integer
- usize - 64 bit unsigned integer
- JsonValue - Embeded json

```rust
struct SubmissionTest {
  id: Uuid, // Id of the test
  submission_id: Uuid, // which submission does this test belong to
  class: String, // Test class- defined and used by support software
  input: String, // Input json as a string
  output: Option<String>, // Output json if any
  correct: Option<bool>, // null if test is in progress, otherwise indicates the correctness of the test
  started_at: String, // The UTC ISO iso date time as a string
  finished_at: Option<String>, // The UTC ISO iso date time as a string, null if in progress
}

struct SubmissionGenerationPayload {
    seed: i64,
    sample_index: Option<i32>,
}

struct SubmissionGenerationResult {
    test_count: i32,
}

struct SubmissionTestGenerationPayload {
    seed: i64,
    test_index: usize,
    sample_index: Option<i32>,
}

struct SubmissionTestGenerationResult {
    input: JsonValue,
    test_class: String,
}

struct VerificationPayload {
    test: SubmissionTest,
    output: JsonValue,
}

struct VerificationResult { 
    correct: bool,
}

interface ProblemSupport {
    fn generate_submission_details(payload: SubmissionGenerationPayload) -> SubmissionGenerationResult;

    fn generate_submission_test_input(payload: SubmissionTestGenerationPayload) -> SubmissionTestGenerationResult;

    fn verify_output(payload: VerificationPayload) -> VerificationResult;
}
```

- Implement these functions as separate api endpoints
- Tell me the routes for these endpoints, all input data is sent as a json, output is required to have the exact schema we expect