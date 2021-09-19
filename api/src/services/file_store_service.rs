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
