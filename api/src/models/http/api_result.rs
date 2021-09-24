use std::fmt::Display;

use okapi::openapi3::Responses;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::util::add_schema_response;
use rocket_okapi::{JsonSchema, OpenApiError};
use serde::Serialize;

#[derive(serde::Serialize, JsonSchema)]
pub enum ApiResult<D, E>
where
    D: Serialize + JsonSchema,
    E: Display,
{
    Ok(D),
    Err(E),
}

pub type AnyApiResult<D> = ApiResult<D, anyhow::Error>;

impl<D, E> From<Result<D, E>> for ApiResult<D, E>
where
    D: Serialize + JsonSchema,
    E: Display,
{
    fn from(res: Result<D, E>) -> Self {
        match res {
            Ok(data) => ApiResult::Ok(data),
            Err(err) => ApiResult::Err(err),
        }
    }
}

impl<D, E> Into<Result<D, E>> for ApiResult<D, E>
where
    D: Serialize + JsonSchema,
    E: Display,
{
    fn into(self) -> Result<D, E> {
        match self {
            ApiResult::Ok(data) => Ok(data),
            ApiResult::Err(err) => Err(err),
        }
    }
}

#[derive(serde::Serialize, JsonSchema)]
pub struct OkErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize, JsonSchema)]
pub struct ErrorResponse {
    pub error: String,
}

impl<'r, D, E> Responder<'r> for ApiResult<D, E>
where
    D: Serialize + JsonSchema,
    E: Display,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self {
            ApiResult::Ok(data) => {
                let mut resp_dat = Json(data).respond_to(req)?;
                resp_dat.set_status(Status::Ok);
                Ok(resp_dat)
            }
            ApiResult::Err(err) => {
                let mut resp_err = Json(ErrorResponse {
                    error: format!("{}", err),
                })
                .respond_to(req)?;
                resp_err.set_status(Status::InternalServerError);
                Ok(resp_err)
            }
        }
    }
}

impl<D: Serialize + JsonSchema, E: Display> OpenApiResponder<'_> for ApiResult<D, E> {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<D>();
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        let schema = gen.json_schema::<ErrorResponse>();
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        Ok(responses)
    }
}
