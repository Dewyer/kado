use crate::models::problem::problem_statement::{ProblemStatement, ProblemStatementDto};
use crate::models::problem::Problem;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct ProblemDto {
    pub id: String,
    pub code_name: String,
    pub name: String,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub max_submissions: i32,
    pub sample_count: i32,
    pub problem_statement_id: String,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub is_deleted: bool,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct ProblemFullyPopulatedDto {
    pub id: String,
    pub code_name: String,
    pub name: String,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub max_submissions: i32,
    pub sample_count: i32,
    pub problem_statement: ProblemStatementDto,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub is_deleted: bool,
}

impl ProblemFullyPopulatedDto {
    pub fn from_problem_and_statement(
        problem: &Problem,
        statement: &ProblemStatement,
    ) -> ProblemFullyPopulatedDto {
        ProblemFullyPopulatedDto {
            id: problem.id.to_string(),
            code_name: problem.code_name.clone(),
            name: problem.name.clone(),
            base_point_value: problem.base_point_value,
            difficulty: problem.difficulty,
            max_submissions: problem.max_submissions,
            sample_count: problem.sample_count,
            problem_statement: statement.to_dto(),
            available_from: problem.available_from.map(|dt| dt.to_string()).clone(),
            available_until: problem.available_until.map(|dt| dt.to_string()).clone(),
            is_deleted: problem.is_deleted,
        }
    }
}
