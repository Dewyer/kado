use crate::guards::{AccessToken, AuthTokenGuard};
use crate::models::http::api_result::AnyApiResult;
use crate::models::http::responses::{GetProblemDetailsResponse, GetProblemsResponse};
use crate::services::problem_service::ProblemService;
use rocket_okapi::openapi;

#[openapi]
#[get("/problems")]
/// Get all available problems
pub fn get_problems(
    _user_guard: AuthTokenGuard<AccessToken>,
    problem_service: ProblemService,
) -> AnyApiResult<GetProblemsResponse> {
    problem_service.get_problems().into()
}

#[openapi]
#[get("/problems/<code_name>")]
/// Get full problem details
pub fn get_problem_details(
    user_guard: AuthTokenGuard<AccessToken>,
    code_name: String,
    problem_service: ProblemService,
) -> AnyApiResult<GetProblemDetailsResponse> {
    problem_service
        .get_problem_details(user_guard, code_name)
        .into()
}
