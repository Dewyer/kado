
#[derive(thiserror::Error, Debug, serde::Serialize)]
pub enum SnapChallengeClientError {
    #[error("FailedToConstructClient")]
    FailedToConstructClient,
    #[error("RequestFailed")]
    RequestFailed,
    #[error("FailedDeserializingResponse")]
    FailedDeserializingResponse,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StartSubmissionPayload {
    pub problem: String,
    pub sample_index: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SubmissionDto {
    pub id: String,
    pub test_count: i32,
    pub sample_index: Option<i32>,
    pub started_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StartSubmissionResponse {
    pub submission: SubmissionDto,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StartTestPayload {
    pub submission: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StartTestResponse<Inp> {
    pub test_id: String,
    pub deadline: i64,
    #[serde(bound(deserialize = "Inp: serde::Deserialize<'de>"))]
    pub input: Inp,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SubmitTestPayload<Out: serde::Serialize> {
    pub test_id: String,
    pub output: Out,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SubmitTestResponse {
    pub correct: bool,
}


pub type ClientResult<T> =  Result<T, SnapChallengeClientError>;
