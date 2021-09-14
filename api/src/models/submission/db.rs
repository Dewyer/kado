use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::submissions;
use crate::models::submission::SubmissionDto;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "submissions"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Submission {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub problem_id: Uuid,
    pub seed: i32,
    pub test_count: i32,
    pub sample_index: Option<i32>,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "submissions"]
pub struct NewSubmission {
    pub owner_id: Uuid,
    pub problem_id: Uuid,
    pub seed: i32,
    pub test_count: i32,
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
            sample_index: self.sample_index.clone(),
            started_at: self.started_at.timestamp(),
            finished_at: self.finished_at.map(|td| td.timestamp()),
        }
    }
}
