use crate::models::problem::ProblemDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetProblemsResponse {
    pub problems: Vec<ProblemDto>,
    pub next_problem_available_at: Option<String>,
}
