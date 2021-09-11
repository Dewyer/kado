use serde::Serialize;
use uuid::Uuid;

use crate::schema::users;
use crate::models::user::UserDto;

#[derive(Queryable, Serialize, AsChangeset)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub authenticator: String,
    pub participate_in_leaderboards: bool,
    pub is_active: bool,
    pub is_admin: bool,
    pub team_id: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub authenticator: &'a str,
    pub participate_in_leaderboards: bool,
    pub is_active: bool,
    pub is_admin: bool,
    pub team_id: Option<Uuid>,
}

impl User {
    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id.to_string(),
            username: self.username.clone(),
            email: self.email.clone(),
            participate_in_leaderboards: self.participate_in_leaderboards.clone(),
            is_admin: self.is_admin.clone(),
            team_id: self.team_id.map(|el| el.to_string()),
        }
    }
}
