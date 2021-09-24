use crate::models::submission::{SubmissionDto, SubmissionMinimalDto};
use crate::schema::submissions;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize, AsChangeset, Clone)]
#[table_name = "submissions"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Submission {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub problem_id: Uuid,
    pub seed: i64,
    pub test_count: i32,
    pub correct: Option<bool>,
    pub proof_file_path: Option<String>,
    pub proof_file_original_name: Option<String>,
    pub sample_index: Option<i32>,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "submissions"]
pub struct NewSubmission {
    pub owner_id: Uuid,
    pub problem_id: Uuid,
    pub seed: i64,
    pub test_count: i32,
    pub correct: Option<bool>,
    pub proof_file_path: Option<String>,
    pub proof_file_original_name: Option<String>,
    pub sample_index: Option<i32>,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

impl Submission {
    pub fn to_dto(&self) -> SubmissionDto {
        SubmissionDto {
            id: self.id.to_string(),
            owner_id: self.owner_id.to_string(),
            problem_id: self.problem_id.to_string(),
            test_count: self.test_count,
            correct: self.correct,
            proof_file_path: self.proof_file_path.clone(),
            proof_file_original_name: self.proof_file_original_name.clone(),
            sample_index: self.sample_index.clone(),
            started_at: self.started_at.timestamp(),
            finished_at: self.finished_at.map(|td| td.timestamp()),
        }
    }

    pub fn to_minimal_dto(&self) -> SubmissionMinimalDto {
        SubmissionMinimalDto {
            id: self.id.to_string(),
            test_count: self.test_count,
            sample_index: self.sample_index.clone(),
            started_at: self.started_at.timestamp(),
        }
    }
}
