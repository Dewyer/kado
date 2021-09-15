#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTestInputRequest {
    pub code_name: String,
}
