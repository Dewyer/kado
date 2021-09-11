use serde::Serialize;
use uuid::Uuid;

use crate::schema::users;
use crate::models::user::UserDto;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub authenticator: String,
    pub participate_in_leaderboards: bool,
    pub is_active: bool,
    pub is_admin: bool,
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
}

impl User {
    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id.to_string(),
            username: self.username.clone(),
            email: self.email.clone(),
            participate_in_leaderboards: self.participate_in_leaderboards.clone(),
            is_admin: self.is_admin.clone(),
        }
    }
}
