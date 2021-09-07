#[derive(serde::Serialize, rocket_okapi::JsonSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}