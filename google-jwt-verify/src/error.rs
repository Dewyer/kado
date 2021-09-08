use crate::algorithm::Algorithm;
use base64::DecodeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidToken,
    RetrieveKeyFailure,
    UnsupportedAlgorithm(Algorithm),
    Expired,
}

impl From<DecodeError> for Error {
    fn from(_: DecodeError) -> Self {
        Error::InvalidToken
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidToken
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(_: ring::error::Unspecified) -> Self {
        Error::InvalidToken
    }
}
