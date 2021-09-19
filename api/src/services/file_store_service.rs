use s3::{Bucket, Region};
use s3::creds::Credentials;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State};
use crate::config::AppConfig;
use rocket::http::Status;
use std::fs;

pub struct FileStoreService {
    config: AppConfig,
}

impl FileStoreService {
    pub fn new(
        config: AppConfig,
    ) -> Self {
        Self {
            config,
        }
    }

    pub fn upload(&self) -> anyhow::Result<()> {
        /*
        let credentials = Credentials::new(ACCESS_KEY,SECRET_KEY,None,None)
            .map_err(|_| anyhow::Error::msg("Cant parse credentials"))?;;
        let region = "eu-central-1".parse().unwrap();

        let bucket = Bucket::new("qpacode", region, credentials)
            .map_err(|_| anyhow::Error::msg("Cant create bucket"))?;
        let file: Vec<u8> = fs::read("file_path");
        let (_, code) = bucket.put("/test.file", &file, "text/plain").unwrap();
        */
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for FileStoreService {
    type Error = ServiceError;

    fn from_request(req: &'a Request<'r>) -> request::Outcome<FileStoreService, Self::Error> {
        let config = req.guard::<State<AppConfig>>().map_failure(|_| {
            (
                Status::InternalServerError,
                ServiceError::ServiceGuardFailed,
            )
        })?;

        request::Outcome::Success(FileStoreService::new(
            config.inner().clone(),
        ))
    }
}
