use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::problems;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "problems"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Problem {
    pub id: Uuid,
    pub code_name: String,
    pub name: String,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub problem_statement_id: Option<Uuid>,
    pub available_from: NaiveDateTime,
    pub available_until: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[table_name = "problems"]
pub struct NewProblem<'a> {
    pub code_name: &'a str,
    pub name: &'a str,
    pub base_point_value: i64,
    pub difficulty: i32,
    pub problem_statement_id: Option<Uuid>,
    pub available_from: NaiveDateTime,
    pub available_until: NaiveDateTime,
    pub is_deleted: bool,
}