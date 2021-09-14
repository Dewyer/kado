#[derive(Clone)]
pub enum CodeName {
    CamelCase,
}

const CAMEL_CASE_NAME: &'static str = "camel-case";

impl CodeName {
    pub fn from_string(code_name: &str) -> anyhow::Result<CodeName> {
        match &code_name.to_lowercase()[..] {
            CAMEL_CASE_NAME => Ok(CodeName::CamelCase),
            _ => { Err(anyhow::Error::msg("Problem id invalid!")) }
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            CodeName::CamelCase => CAMEL_CASE_NAME.to_string(),
        }
    }
}