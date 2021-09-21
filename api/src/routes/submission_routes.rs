use rocket_okapi::openapi;
use crate::models::http::api_result::{AnyApiResult, ErrorResponse, OkErrorResponse};
use crate::guards::{ApiToken, AuthTokenGuard, AccessToken};
use crate::services::submission::submission_service::SubmissionService;
use rocket_contrib::json::Json;
use crate::models::http::requests::{StartSubmissionRequest, SendTestOutputRequest, GetTestInputRequest};
use crate::models::http::responses::{StartSubmissionResponse, GetTestInputResponse, SendTestOutputResponse};
use rocket::Data;
use rocket::http::ContentType;
use crate::models::http::uploaded_file::{UploadedFile, ZipFile};

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

#[post("/submissions/code-upload/<problem_code>", data="<file>")]
pub fn upload_proof_api(
    problem_code: String,
    file: UploadedFile<ZipFile>,
    user_guard: AuthTokenGuard<ApiToken>,
    submission_service: SubmissionService,
) -> Json<OkErrorResponse> {
    let res = submission_service.upload_proof(&mut user_guard.user.clone(), file, problem_code);
    Json(OkErrorResponse {
        error: if let Err(err) = res { format!("{}", err) } else { "".to_string() },
    })
}


#[post("/submissions/code-upload-with-token/<problem_code>", data = "<file>")]
pub fn upload_proof_frontend(
    problem_code: String,
    file: UploadedFile<ZipFile>,
    user_guard: AuthTokenGuard<AccessToken>,
    submission_service: SubmissionService,
) -> Json<OkErrorResponse> {
    let res = submission_service.upload_proof(&mut user_guard.user.clone(), file, problem_code);
    Json(OkErrorResponse {
        error: if let Err(err) = res { format!("{}", err) } else { "".to_string() },
    })
}