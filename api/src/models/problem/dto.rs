#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct ProblemDto {
    pub id: String,
    pub code_name: String,
    pub name: String,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub problem_statement_id: String,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub is_deleted: bool,
}