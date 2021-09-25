use crate::blackjack_hand::BlackjackHand;
use uuid::Uuid;
use crate::errors::{BlackGameResult, BlackjackGameError};

pub struct BlackjackPlayer {
    pub id: Uuid,
    pub plays_with_credits: bool,
    pub credits: usize,
    pub hands: Vec<BlackjackHand>,
}

impl BlackjackPlayer {
    pub fn assert_has_more_credits_then(&self, credits: usize) -> BlackGameResult<()> {
        if self.credits >= credits {
            Ok(())
        } else {
            Err(BlackjackGameError::PlayerNotEnoughCredits(credits))
        }
    }
}