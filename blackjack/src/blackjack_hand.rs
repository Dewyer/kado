use crate::playing_card::PlayingCard;

pub struct BlackjackHand {
    pub bet: usize,
    pub cards: Vec<PlayingCard>,
}

impl BlackjackHand {
    pub fn from_bet(bet: usize) -> Self {
        Self {
            bet,
            cards: vec![],
        }
    }
}