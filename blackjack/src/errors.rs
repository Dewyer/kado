use crate::models::BlackjackGameStatus;
use crate::blackjack_game_actions::BlackjackGameAction;

#[derive(thiserror::Error, Debug)]
pub enum BlackjackGameError {
    #[error("InvalidPlayingCardIndex")]
    InvalidPlayingCardIndex,
    #[error("PlayerNotPartOfRound")]
    PlayerNotPartOfRound,
    #[error("InvalidSuiteIndex")]
    InvalidSuiteIndex,
    #[error("Invalid action for this game status")]
    InvalidActionForGameStatus(BlackjackGameStatus),
    #[error("Player with this id not found")]
    PlayerWithIdNotFound,
    #[error("Player doesn't have at least `{0}` credits")]
    PlayerNotEnoughCredits(usize),
}

pub type BlackGameResult<T> = Result<T, BlackjackGameError>;