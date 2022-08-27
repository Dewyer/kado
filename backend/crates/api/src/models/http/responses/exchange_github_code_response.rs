#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct ExchangeGithubCodeResponse {
    pub token: String,
}
