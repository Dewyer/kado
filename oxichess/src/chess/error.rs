use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ChessError
{
    InvalidMove,
    NotRightPlayersMove,
    InvalidPiecePosition,
    PieceIsNotYours,
    CantParseNotation
}

impl Display for ChessError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl Error for ChessError
{}