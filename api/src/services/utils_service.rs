use uuid::Uuid;
use rocket::Request;
use chrono::{Utc, NaiveDateTime};

pub const AUTHORIZATION_HEADER: &'static str = "Authorization";

pub struct UtilsService {}

impl UtilsService {
    pub fn naive_now() -> chrono::NaiveDateTime {
        let now = Utc::now();
        NaiveDateTime::from_timestamp(now.timestamp(),0)
    }

    pub fn time_within_seconds(time: chrono::NaiveDateTime, seconds: i64) -> bool {
        let now = Self::naive_now();

        (now.timestamp() - time.timestamp()).abs() <= seconds
    }

    pub fn parse_uuid(id_str: &str) -> anyhow::Result<Uuid> {
        Uuid::parse_str(id_str)
            .map(|v| v)
            .map_err(|_| anyhow::Error::msg("Uui given is invalid!"))
    }

    pub fn parse_optional_uuid(id_str: &Option<String>) -> anyhow::Result<Option<Uuid>> {
        if let Some(id_some_str) = id_str.as_ref() {
            Self::parse_uuid(id_some_str).map(|v| Some(v))
        } else {
            Ok(None).into()
        }
    }

    pub fn header_value_or_empty_from_request(req: &Request<'_>, hdr: &str) -> String {
        req.headers().get(hdr).next().unwrap_or("").to_string()
    }

    pub fn get_bearer_token_from_header(req: &Request<'_>) -> String {
        let raw_val = UtilsService::header_value_or_empty_from_request(req, AUTHORIZATION_HEADER);
        raw_val
            .split("Bearer ")
            .skip(1)
            .next()
            .unwrap_or("")
            .to_string()
    }
}
