use crate::models::http::naive_date_time::ApiNaiveDateTime;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct RecordSleepStatusRequest {
    pub status: String,
}
