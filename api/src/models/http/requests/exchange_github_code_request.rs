#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct ExchangeGithubCodeRequest {
    pub code: String,
}
