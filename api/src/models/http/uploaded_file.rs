use rocket::request::FromRequest;
use crate::errors::ServiceError;
use rocket::{Request, request, Data, data};
use rocket::http::{ContentType, Status};
use multipart_any::server::{Multipart, Entries, SaveResult};
use multipart_any::server::save::SaveResult::{Full, Partial, Error};
use multipart_any::server::save::SavedData;
use std::sync::Arc;
use rocket::data::{FromDataSimple , Transform, Outcome};
use std::path::Path;
use crate::services::crypto_service::CryptoService;

pub struct UploadedFile {
    pub local_path: String,
}

impl UploadedFile {
    pub fn new(local_path: String) -> Self {
        Self {
            local_path,
        }
    }
}

fn process_entries(entries: Entries) -> anyhow::Result<UploadedFile> {
    let files = entries.fields
        .into_iter()
        .find(|el| el.0 == Arc::from("file"))
        .map(|el| el.1)
        .ok_or_else(|| anyhow::Error::msg("no file field!"))?;
    let file = files.into_iter().next().ok_or_else(|| anyhow::Error::msg("no file field!"))?;

    if let SavedData::File(file_data, _) = file.data {
        if std::fs::metadata("./temp").is_err() {
            std::fs::create_dir("./temp");
        }

        let temp_f = format!("./temp/{}.zip", CryptoService::get_random_string(8));
        std::fs::copy(file_data.clone(), Path::new(&temp_f));
        
        Ok(UploadedFile::new(temp_f))
    } else {
        Err(anyhow::Error::msg("Not a file or a zip or good, actually you are just bad!"))
    }
}

fn process_upload(boundary: &str, data: Data) -> anyhow::Result<UploadedFile> {
    Ok(match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries)?,
        Partial(_, _) => {
            return Err(anyhow::Error::msg("Partial request processing"));
        },
        Error(e) => return Err(anyhow::Error::msg("Failed to multipart form!")),
    })
}

impl FromDataSimple for UploadedFile {
    type Error = ServiceError;

    fn from_data(req: &Request, data: Data) -> data::Outcome<UploadedFile, Self::Error> {
        let cont_type = req.guard::<&ContentType>();
        if let request::Outcome::Success(cont) = cont_type {
            let res = cont.params().find(|&(k, _)| k == "boundary");
            if let Some((_, boundary)) = res {
                return match process_upload(boundary, data) {
                    Ok(resp) => data::Outcome::Success(resp),
                    Err(_) => data::Outcome::Failure((
                        Status::InternalServerError,
                        ServiceError::ServiceGuardFailed,
                    ))
                };
            }
        }

        data::Outcome::Failure((
            Status::InternalServerError,
            ServiceError::ServiceGuardFailed,
        ))
    }
}
