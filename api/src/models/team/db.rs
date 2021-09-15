use serde::Serialize;
use uuid::Uuid;
use crate::schema::teams;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "teams"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub join_code: String,
    pub points: i64,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub join_code: &'a str,
    pub points: i64,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
}
