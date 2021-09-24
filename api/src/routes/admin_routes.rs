use crate::guards::AdminKeyGuard;
use crate::models::http::api_result::AnyApiResult;
use crate::models::http::requests::UpdateProblemStatementRequest;
use crate::services::problem_service::ProblemService;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[post("/admin/update-problem-statement", format = "json", data = "<request>")]
/// Update problem statement
pub fn update_problem_statement(
    _admin: AdminKeyGuard,
    request: Json<UpdateProblemStatementRequest>,
    problem_service: ProblemService,
) -> AnyApiResult<()> {
    problem_service
        .update_problem_statement_by_admin(request.0)
        .into()
}
