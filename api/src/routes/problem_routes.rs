use rocket_contrib::json::Json;
use rocket_okapi::openapi;
use crate::models::http::api_result::AnyApiResult;
use crate::guards::{AccessToken, AuthTokenGuard};
use crate::services::problem_service::ProblemService;
use crate::models::http::responses::GetProblemsResponse;

#[openapi]
#[get("/problems")]
/// Get all available problems
pub fn get_problems(
    _user_guard: AuthTokenGuard<AccessToken>,
    problem_service: ProblemService,
) -> AnyApiResult<GetProblemsResponse> {
    problem_service
        .get_problems()
        .into()
}