#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTestInputResponse {
    pub test_id: String,
    pub deadline: i64,
    pub input: serde_json::Value,
}
