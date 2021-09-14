use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{ApiToken, AuthTokenGuard};
use crate::services::submission::submission_service::SubmissionService;
use rocket_contrib::json::Json;
use crate::models::http::requests::StartSubmissionRequest;
use crate::models::http::responses::StartSubmissionResponse;

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
