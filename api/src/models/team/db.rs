use serde::Serialize;
use uuid::Uuid;
use crate::schema::teams;

#[derive(Queryable, Serialize, AsChangeset)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub join_code: String,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub join_code: &'a str,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
}
