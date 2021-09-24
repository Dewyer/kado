use crate::schema::submission_tests;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize, AsChangeset, Clone, serde::Deserialize)]
#[table_name = "submission_tests"]
#[changeset_options(treat_none_as_null = "true")]
pub struct SubmissionTest {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub class: String,
    pub input: String,
    pub output: Option<String>,
    pub correct: Option<bool>,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "submission_tests"]
pub struct NewSubmissionTest<'a> {
    pub submission_id: Uuid,
    pub class: &'a str,
    pub input: &'a str,
    pub output: Option<&'a str>,
    pub correct: Option<bool>,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}
