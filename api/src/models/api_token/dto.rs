#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema, Clone)]
pub struct ApiTokenDto {
    pub id: String,
    pub token: String,
    pub owner_id: String,
    pub is_deleted: bool,
}
