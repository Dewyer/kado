#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTestInputRequest {
    pub submission: String,
}
