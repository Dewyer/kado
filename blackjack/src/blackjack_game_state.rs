use crate::blackjack_player::BlackjackPlayer;
use crate::blackjack_hand::BlackjackHand;
use crate::models::BlackjackGameStatus;

pub trait BlackjackGameStateProvider {
    fn set_status(&mut self, status: BlackjackGameStatus);

    fn get_status(&self) -> &BlackjackGameStatus;

    fn set_card_index(&mut self, card: usize);

    fn get_card_index(&self) -> usize;

    fn set_turn_index(&mut self, turn: usize);

    fn get_turn_index(&self) -> usize;

    fn get_dealer_hand_mut(&mut self) -> &mut BlackjackHand;

    fn get_players_mut(&mut self) -> &mut Vec<BlackjackPlayer>;
}

pub type IBlackjackGameStateProvider = Box<dyn BlackjackGameStateProvider>;

pub struct InMemoryBlackjackGameState {
    pub status: BlackjackGameStatus,

    pub turn_index: usize,
    pub card_index: usize,

    pub dealer_hand: BlackjackHand,
    pub players: Vec<BlackjackPlayer>,
}

impl BlackjackGameStateProvider for InMemoryBlackjackGameState {
    fn set_status(&mut self, status: BlackjackGameStatus) {
        self.status = status;
    }

    fn get_status(&self) -> &BlackjackGameStatus {
        &self.status
    }

    fn set_card_index(&mut self, card: usize) {
        self.card_index = card;
    }

    fn get_card_index(&self) -> usize {
        self.card_index
    }

    fn set_turn_index(&mut self, turn: usize) {
        self.turn_index = turn;
    }

    fn get_turn_index(&self) -> usize {
        self.turn_index
    }

    fn get_dealer_hand_mut(&mut self) -> &mut BlackjackHand {
        &mut self.dealer_hand
    }

    fn get_players_mut(&mut self) -> &mut Vec<BlackjackPlayer> {
        &mut self.players
    }
}