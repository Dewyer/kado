#[derive(Clone)]
pub enum CodeName {
    SanityCheck,
}

const SANITY_CHECK_NAME: &'static str = "sanity-check";

impl CodeName {
    pub fn from_string(code_name: &str) -> anyhow::Result<CodeName> {
        match &code_name.to_lowercase()[..] {
            SANITY_CHECK_NAME => Ok(CodeName::SanityCheck),
            _ => Err(anyhow::Error::msg("Problem id invalid!")),
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            CodeName::SanityCheck => SANITY_CHECK_NAME.to_string(),
        }
    }
}
