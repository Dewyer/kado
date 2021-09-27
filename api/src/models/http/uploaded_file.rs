use crate::errors::{ServiceError, UploadFileError};
use crate::services::crypto_service::CryptoService;
use multipart_any::server::save::SaveResult::{Error, Full, Partial};
use multipart_any::server::save::SavedData;
use multipart_any::server::{Entries, Multipart, SaveResult};
use rocket::data::{FromDataSimple, Outcome, Transform};
use rocket::http::{ContentType, Status};
use rocket::request::FromRequest;
use rocket::{data, request, Data, Request};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use log::{error};

pub trait FileMimeTrait {
    fn content_type() -> String;

    fn name() -> String;
}

pub struct ZipFile {}

impl FileMimeTrait for ZipFile {
    fn content_type() -> String {
        "application/zip".to_string()
    }

    fn name() -> String {
        "zip".to_string()
    }
}

pub struct UploadedFile<Mime: FileMimeTrait> {
    pub local_path: String,
    mime: PhantomData<Mime>,
}

impl<Mime: FileMimeTrait> UploadedFile<Mime> {
    pub fn new(local_path: String) -> Self {
        Self {
            local_path,
            mime: PhantomData::default(),
        }
    }

    fn save_longer_temporarily_bytes(bytes: Vec<u8>) -> anyhow::Result<String> {
        if std::fs::metadata("./temp").is_err() {
            std::fs::create_dir("./temp")
                .map_err(|_| anyhow::Error::msg("Temp directory can't be created"))?;
        }

        let temp_f = format!("./temp/{}.zip", CryptoService::get_random_string(8));
        std::fs::write(Path::new(&temp_f), bytes)
            .map_err(|_| anyhow::Error::msg("File can't be saved to temp directory!"))?;

        Ok(temp_f)
    }

    fn save_longer_temporarily(temp_temp_path: PathBuf) -> anyhow::Result<String> {
        if std::fs::metadata("./temp").is_err() {
            std::fs::create_dir("./temp")
                .map_err(|_| anyhow::Error::msg("Temp directory can't be created"))?;
        }

        let temp_f = format!("./temp/{}.zip", CryptoService::get_random_string(8));
        std::fs::copy(temp_temp_path, Path::new(&temp_f))
            .map_err(|_| anyhow::Error::msg("File can't be saved to temp directory!"))?;

        Ok(temp_f)
    }

    fn from_entries(entries: Entries) -> anyhow::Result<UploadedFile<Mime>> {
        let files = entries
            .fields
            .into_iter()
            .find(|el| el.0 == Arc::from("file"))
            .map(|el| el.1)
            .ok_or_else(|| anyhow::Error::msg("no file field!"))?;
        let file = files
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::Error::msg("no file field!"))?;

        match file.data {
            SavedData::File(file_path_buf, _) => {
                let kind = infer::get_from_path(file_path_buf.clone())
                    .map_err(|_| anyhow::Error::msg("File type couldn't be identified."))?
                    .ok_or_else(|| anyhow::Error::msg("File type unknown"))?;

                if kind.mime_type() != Mime::content_type() {
                    return Err(anyhow::Error::msg(format!(
                        "File type not {}",
                        Mime::name()
                    )));
                }

                let temp_f = Self::save_longer_temporarily(file_path_buf)?;
                Ok(UploadedFile::new(temp_f))
            },
            SavedData::Bytes(bytes) => {
                let kind = infer::get(&bytes)
                    .ok_or_else(|| anyhow::Error::msg("File type unknown"))?;

                if kind.mime_type() != Mime::content_type() {
                    return Err(anyhow::Error::msg(format!(
                        "File type not {}",
                        Mime::name()
                    )));
                }

                let temp_f = Self::save_longer_temporarily_bytes(bytes)?;
                Ok(UploadedFile::new(temp_f))
            },
            _ => Err(anyhow::Error::msg("Not a file!"))
        }
    }

    pub fn from_multipart_data(boundary: &str, data: Data) -> anyhow::Result<UploadedFile<Mime>> {
        Ok(
            match Multipart::with_body(data.open(), boundary).save().temp() {
                Full(entries) => Self::from_entries(entries)?,
                Partial(_, _) => {
                    return Err(anyhow::Error::msg("Partial request processing"));
                }
                Error(_) => return Err(anyhow::Error::msg("Failed to multipart form!")),
            },
        )
    }
}

impl<Mime: FileMimeTrait> FromDataSimple for UploadedFile<Mime> {
    type Error = UploadFileError;

    fn from_data(req: &Request, data: Data) -> data::Outcome<UploadedFile<Mime>, Self::Error> {
        let cont_type = req.guard::<&ContentType>();
        if let request::Outcome::Success(cont) = cont_type {
            let res = cont.params().find(|&(k, _)| k == "boundary");
            if let Some((_, boundary)) = res {
                return match UploadedFile::<Mime>::from_multipart_data(boundary, data) {
                    Ok(resp) => data::Outcome::Success(resp),
                    Err(_) => data::Outcome::Failure((
                        Status::BadRequest,
                        UploadFileError::FileUploadFailed,
                    )),
                };
            }
        }

        data::Outcome::Failure((Status::BadRequest, UploadFileError::FileUploadFailed))
    }
}
