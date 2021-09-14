#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct SendTestOutputResponse {
    pub correct: bool,
}