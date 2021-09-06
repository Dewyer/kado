use chrono::NaiveDateTime;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[get("/health")]
/// Healthcheck
pub fn healthcheck_route() -> Json<()> {
    Json(())
}
