use crate::blackjack_hand::BlackjackHand;
use uuid::Uuid;
use crate::errors::{BlackGameResult, BlackjackGameError};

pub struct BlackjackPlayer {
    pub id: Uuid,
    pub plays_with_credits: bool,
    pub credits: usize,

    pub done_playing: bool,
    pub doubled_down: Option<bool>,
    pub has_insurance: Option<bool>,
    pub bet: Option<usize>,
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

    pub fn can_double_down(&self) -> bool {
        let double_down_values = vec![9usize, 10usize, 11usize];
        self.hands
            .get(0)
            .map(|hand|
                hand
                    .get_hand_values()
                    .iter()
                    .any(|vv| double_down_values.iter().any(|el| el == vv))
            ).unwrap_or(false)
    }

    pub fn has_natural_blackjack(&self) -> bool {
        self.hands
            .get(0)
            .map(|hand|
                hand.has_blackjack()
            ).unwrap_or(false)
    }
}