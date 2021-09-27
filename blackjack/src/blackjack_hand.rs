use crate::playing_card::PlayingCard;
use std::collections::HashSet;

pub struct BlackjackHand {
    pub cards: Vec<PlayingCard>,
}

impl BlackjackHand {
    pub fn new() -> Self {
        Self {
            cards: vec![],
        }
    }

    pub fn could_have_blackjack(&self) -> bool {
        self.cards
            .iter()
            .next()
            .map(|card| card.is_ace_or_ten())
            .unwrap_or(false)
    }

    pub fn has_blackjack(&self) -> bool {
        self.get_hand_values().iter().any(|vv| vv == 21)
    }

    pub fn get_hand_values(&self) -> Vec<usize> {
        let mut values: Vec<usize> = vec![];

        for ii in self.cards.iter() {
            let card_values = ii.get_values();
            let mut new_values = vec![];

            for cv in card_values.into_iter() {
                for vv in values.iter() {
                    new_values.push(vv + cv);
                }
            }

            values = new_values;
        }

        let mut diff_values = HashSet::new();
        let mut final_values = vec![];

        for ii in values.into_iter() {
            if !diff_values.contains(&ii) {
                diff_values.insert(ii);
                final_values.push(ii);
            }
        }

        final_values
    }
}