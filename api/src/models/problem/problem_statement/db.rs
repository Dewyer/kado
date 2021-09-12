use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::problem_statements;
use serde::Serialize;
use crate::models::problem::problem_statement::ProblemStatementDto;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "problem_statements"]
#[changeset_options(treat_none_as_null = "true")]
pub struct ProblemStatement {
    pub id: Uuid,
    pub version: Option<String>,
    pub problem_statement: String,
}

#[derive(Insertable)]
#[table_name = "problem_statements"]
pub struct NewProblemStatement<'a> {
    pub version: &'a str,
    pub problem_statement: &'a str,
}

impl ProblemStatement {
    pub fn to_dto(&self) -> ProblemStatementDto {
        ProblemStatementDto {
            id: self.id.to_string(),
            version: self.version.clone(),
            problem_statement: self.problem_statement.clone(),
        }
    }
}