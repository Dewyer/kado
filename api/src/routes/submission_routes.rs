use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{ApiToken, AuthTokenGuard};
use crate::services::submission::submission_service::SubmissionService;
use rocket_contrib::json::Json;
use crate::models::http::requests::{StartSubmissionRequest, SendTestOutputRequest, GetTestInputRequest};
use crate::models::http::responses::{StartSubmissionResponse, GetTestInputResponse, SendTestOutputResponse};

#[openapi]
#[post("/submissions/start-submission", format = "json", data = "<request>")]
/// Start a new submission
pub fn start_submission(
    user_guard: AuthTokenGuard<ApiToken>,
    request: Json<StartSubmissionRequest>,
    submission_service: SubmissionService,
) -> AnyApiResult<StartSubmissionResponse> {
    submission_service
        .start_submission(user_guard, request.0)
        .into()
}

#[openapi]
#[put("/submissions/test", format = "json", data = "<request>")]
/// Get input for a new test
pub fn get_test_input(
    user_guard: AuthTokenGuard<ApiToken>,
    request: Json<GetTestInputRequest>,
    submission_service: SubmissionService,
) -> AnyApiResult<GetTestInputResponse> {
    submission_service
        .get_test_input(user_guard, request.0)
        .into()
}

#[openapi]
#[post("/submissions/test/<test_id>", format = "json", data = "<request>")]
/// Send output for an existing test
pub fn send_test_output(
    user_guard: AuthTokenGuard<ApiToken>,
    test_id: String,
    request: Json<SendTestOutputRequest>,
    submission_service: SubmissionService,
) -> AnyApiResult<SendTestOutputResponse> {
    submission_service
        .send_test_output(user_guard, test_id, request.0)
        .into()
}
