use thiserror::Error;

#[derive(Error, Debug, serde::Serialize)]
pub enum ServiceError {
    #[error("ServiceGuardFailed")]
    ServiceGuardFailed,
}

#[derive(Error, Debug, serde::Serialize)]
pub enum AuthError {
    #[error("AdminAuthError")]
    AdminAuthError,
    #[error("ApiAuthError")]
    ApiAuthError,
}
