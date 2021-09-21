#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct SubmissionDto {
    pub id: String,
    pub owner_id: String,
    pub problem_id: String,
    pub test_count: i32,
    pub correct: Option<bool>,
    pub proof_file_path: Option<String>,
    pub proof_file_original_name: Option<String>,
    pub sample_index: Option<i32>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct SubmissionMinimalDto {
    pub id: String,
    pub test_count: i32,
    pub sample_index: Option<i32>,
    pub started_at: i64,
}