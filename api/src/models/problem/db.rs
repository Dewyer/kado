use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::problems;
use serde::Serialize;
use crate::models::problem::ProblemDto;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "problems"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Problem {
    pub id: Uuid,
    pub code_name: String,
    pub name: String,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub max_submissions: i32,
    pub sample_count: i32,
    pub problem_statement_id: Uuid,
    pub available_from: Option<NaiveDateTime>,
    pub available_until: Option<NaiveDateTime>,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[table_name = "problems"]
pub struct NewProblem<'a> {
    pub code_name: &'a str,
    pub name: &'a str,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub max_submissions: i32,
    pub sample_count: i32,
    pub problem_statement_id: Uuid,
    pub available_from: Option<NaiveDateTime>,
    pub available_until: Option<NaiveDateTime>,
    pub is_deleted: bool,
}

impl Problem {
    pub fn to_dto(&self) -> ProblemDto {
        ProblemDto {
            id: self.id.to_string(),
            code_name: self.code_name.clone(),
            name: self.name.clone(),
            base_point_value: self.base_point_value,
            difficulty: self.difficulty,
            max_submissions: self.max_submissions,
            sample_count: self.sample_count,
            problem_statement_id: self.problem_statement_id.to_string(),
            available_from: self.available_from.map(|dt| dt.to_string()).clone(),
            available_until: self.available_until.map(|dt| dt.to_string()).clone(),
            is_deleted: self.is_deleted,
        }
    }
}