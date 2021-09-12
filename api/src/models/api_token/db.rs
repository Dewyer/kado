use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::api_tokens;
use serde::Serialize;
use crate::models::api_token::ApiTokenDto;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "api_tokens"]
#[changeset_options(treat_none_as_null = "true")]
pub struct ApiToken {
    pub id: Uuid,
    pub token: String,
    pub owner_id: Uuid,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[table_name = "api_tokens"]
pub struct NewApiToken<'a> {
    pub token: &'a str,
    pub owner_id: Uuid,
    pub is_deleted: bool,
}

impl ApiToken {
    pub fn to_dto(&self) -> ApiTokenDto {
        ApiTokenDto {
            id: self.id.to_string(),
            token: self.token.clone(),
            owner_id: self.owner_id.to_string(),
            is_deleted: self.is_deleted,
        }
    }
}