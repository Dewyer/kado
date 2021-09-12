#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct ProblemStatementDto {
    pub id: String,
    pub version: Option<String>,
    pub problem_statement: String,
}