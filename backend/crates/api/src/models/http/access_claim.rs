#[derive(serde::Serialize, serde::Deserialize)]
pub struct AccessClaim {
    pub user_id: String,
    pub expires_at: u64,
}
