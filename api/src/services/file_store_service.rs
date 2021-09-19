use s3::{Bucket, Region};
use s3::creds::Credentials;
use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, State, Data};
use crate::config::AppConfig;
use rocket::http::Status;
use std::path::Path;
use crate::services::crypto_service::CryptoService;
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

    pub fn store_file(&self, file_name: &str, temp_path: &str) -> anyhow::Result<()> {
        let credentials = Credentials::new(Some(&self.config.aws_access_key), Some(&self.config.aws_secret_key),None,None, None)
            .map_err(|_| anyhow::Error::msg("Cant parse credentials"))?;
        let region = "eu-central-1".parse().unwrap();

        let bucket = Bucket::new(&self.config.aws_bucket, region, credentials)
            .map_err(|_| anyhow::Error::msg("Cant create bucket"))?;

        let temp_name = Path::new(&temp_path);
        let cont = fs::read(temp_name.clone()).map_err(|_| anyhow::Error::msg("read err"))?;

        let (_, code) = bucket.put_object_blocking(&file_name,&cont[..])
            .map_err(|err| {
                println!("internal err: {:?}", err);

                anyhow::Error::msg("Cant upload local file")
            })?;

        if code != 200 {
            bail!("Failed to upload file!");
        }

        Ok(())
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
