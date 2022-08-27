#[derive(Clone)]
pub enum CodeName {
    SanityCheck,
    Maze,
    KingPinned,
    Barrel,
}

const SANITY_CHECK_NAME: &'static str = "sanity-check";
const MAZE_NAME: &'static str = "maze";
const KING_PINNED_NAME: &'static str = "king-pinned";
const BARREL_NAME: &'static str = "barrel";

impl CodeName {
    pub fn from_string(code_name: &str) -> anyhow::Result<CodeName> {
        match &code_name.to_lowercase()[..] {
            SANITY_CHECK_NAME => Ok(CodeName::SanityCheck),
            MAZE_NAME => Ok(CodeName::Maze),
            KING_PINNED_NAME => Ok(CodeName::KingPinned),
            BARREL_NAME => Ok(CodeName::Barrel),
            _ => Err(anyhow::Error::msg("Problem id invalid!")),
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            CodeName::Maze => MAZE_NAME.to_string(),
            CodeName::SanityCheck => SANITY_CHECK_NAME.to_string(),
            CodeName::KingPinned => KING_PINNED_NAME.to_string(),
            CodeName::Barrel => BARREL_NAME.to_string(),
        }
    }
}
