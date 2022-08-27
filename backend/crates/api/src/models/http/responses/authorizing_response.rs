use crate::models::user::UserDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct AuthorizingResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserDto,
}
