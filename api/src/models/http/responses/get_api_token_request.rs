use crate::models::api_token::ApiTokenDto;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetApiTokenResponse {
    pub token: ApiTokenDto,
}
