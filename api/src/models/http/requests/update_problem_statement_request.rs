#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct UpdateProblemStatementRequest {
    pub problem: String,
    pub statement: String,
    pub version: String,
}
