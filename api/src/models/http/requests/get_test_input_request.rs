#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct GetTestInputRequest {
    pub problem: String,
}
