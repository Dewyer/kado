#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTestInputResponse {
    pub input: serde_json::Value,
}
