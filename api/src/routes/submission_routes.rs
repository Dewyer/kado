use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{ApiToken, AuthTokenGuard};
use crate::services::submission::submission_service::SubmissionService;
use rocket_contrib::json::Json;
use crate::models::http::requests::StartSubmissionRequest;
use crate::models::http::responses::{StartSubmissionResponse, GetTestInputResponse};

#[openapi]
#[post("/start-submission", format = "json", data = "<request>")]
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
#[post("/get-test-input/<code_name>")]
/// Start a new submission
pub fn get_test_input(
    user_guard: AuthTokenGuard<ApiToken>,
    code_name: String,
    submission_service: SubmissionService,
) -> AnyApiResult<GetTestInputResponse> {
    submission_service
        .get_test_input(user_guard, code_name)
        .into()
}
