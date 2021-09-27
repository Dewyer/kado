#[derive(Clone)]
pub enum CodeName {
    SanityCheck,
    Maze,
}

const SANITY_CHECK_NAME: &'static str = "sanity-check";
const MAZE_NAME: &'static str = "maze";

impl CodeName {
    pub fn from_string(code_name: &str) -> anyhow::Result<CodeName> {
        match &code_name.to_lowercase()[..] {
            SANITY_CHECK_NAME => Ok(CodeName::SanityCheck),
            MAZE_NAME => Ok(CodeName::Maze),
            _ => Err(anyhow::Error::msg("Problem id invalid!")),
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            CodeName::Maze => MAZE_NAME.to_string(),
            CodeName::SanityCheck => SANITY_CHECK_NAME.to_string(),
        }
    }
}
