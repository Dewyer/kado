#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct SendTestOutputRequest {
    pub output: serde_json::Value,
}
