use crate::errors::{BlackGameResult, BlackjackGameError};

pub const DECK_SIZE: usize = 52usize;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PlayingCardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl PlayingCardSuit {
    pub fn from_index(idx: usize) -> BlackGameResult<PlayingCardSuit> {
        if idx >= 4 {
            return Err(BlackjackGameError::InvalidSuiteIndex);
        }

        Ok(match idx {
            0 => PlayingCardSuit::Clubs,
            1 => PlayingCardSuit::Diamonds,
            2 => PlayingCardSuit::Hearts,
            _ => PlayingCardSuit::Spades,
        })
    }
}

#[derive(Clone)]
pub struct PlayingCard(usize);

impl PlayingCard {
    pub fn from_index(card_index: usize) -> BlackGameResult<Self> {
        if card_index >= DECK_SIZE {
            return Err(BlackjackGameError::InvalidPlayingCardIndex);
        }

        Ok(Self(card_index))
    }

    pub fn get_suite_and_type(&self) -> (PlayingCardSuit, usize) {
        let idx = self.0 % 13;
        let suite_index = (self.0 - idx) / 13;

        (PlayingCardSuit::from_index(suite_index).expect("Suite should be correct!"), idx)
    }

    pub fn get_values(&self) -> Vec<usize> {
        let info =  self.get_suite_and_type();
        match info.1 {
            0..9 => vec![info.1+2],
            12 => vec![1,11],
            _ => vec![10],
        }
    }

    pub fn is_ace_or_ten(&self) -> bool {
        let val = self.get_suite_and_type().1;
        vec![8, 9, 10, 11, 12].iter().any(|el| el == val)
    }
}

#[cfg(test)]
mod tests {
    use crate::playing_card::{PlayingCard, PlayingCardSuit};

    #[test]
    fn from_index_works() {
        let card = PlayingCard::from_index(15)
            .unwrap();

        assert_eq!(card.get_suite_and_type().0, PlayingCardSuit::Diamonds);
        assert_eq!(card.get_suite_and_type().1, 2);
        assert_eq!(card.get_values().iter().next().unwrap().clone(), 4);
    }

    #[test]
    fn get_values_works_non_ace() {
        let card = PlayingCard::from_index(21)
            .unwrap();

        assert_eq!(card.get_values().iter().next().unwrap().clone(), 9);
    }

    #[test]
    fn get_values_works_on_ace() {
        let card = PlayingCard::from_index(12)
            .unwrap();
        let values = card.get_values();
        let mut values_iter = values.iter();
        assert_eq!(values_iter.next().unwrap().clone(), 1);
        assert_eq!(values_iter.next().unwrap().clone(), 11);
    }
}