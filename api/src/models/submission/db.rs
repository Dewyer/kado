use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::submissions;

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