use uuid::Uuid;

pub struct UtilsService {}

impl UtilsService {
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
}
