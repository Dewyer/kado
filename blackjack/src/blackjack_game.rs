use crate::blackjack_game_state::IBlackjackGameStateProvider;
use crate::rng::SeededRng;
use crate::errors::{BlackGameResult, BlackjackGameError};
use crate::models::{BlackjackGameStatus, BlackjackGameConfig};
use crate::blackjack_game_actions::{BlackjackGameAction, BetAction, InsuranceBetAction};
use crate::blackjack_player::BlackjackPlayer;
use uuid::Uuid;
use crate::blackjack_hand::BlackjackHand;
use crate::playing_card::{PlayingCard, DECK_SIZE};

pub struct BlackjackGame {
    rng: SeededRng,
    state: IBlackjackGameStateProvider,
    config: BlackjackGameConfig,

    deck: Vec<PlayingCard>,
}

impl BlackjackGame {
    pub fn new(state: IBlackjackGameStateProvider, config: BlackjackGameConfig) -> Self {
        let mut game = Self {
            state,
            rng: SeededRng::new(config.seed),
            config,
        };
        game.generate_deck();

        game
    }

    fn generate_deck(&mut self) {
        for _ in 0..(self.config.deck_count * DECK_SIZE) {
            self.deck.push(PlayingCard::from_index(self.rng.gen_range(0, DECK_SIZE)).unwrap());
        }

        for ii in 0..self.deck.len() {
            let swap_index = self.rng.gen_range(0, self.deck.len());
            let temp = self.deck[swap_index].clone();
            self.deck[swap_index] = self.deck[ii].clone();
            self.deck[ii] = temp;
        }
    }

    fn draw_card(&mut self) -> PlayingCard {
        let ci = self.state.get_card_index();
        self.state.set_card_index(ci+1);

        self.deck[ci+1].clone()
    }

    pub fn reset_state(&mut self) -> () {
        self.state.set_status(BlackjackGameStatus::Betting);
        self.state.set_turn_index(0);
        self.state.set_card_index(0);

        let dealer = self.state.get_dealer_hand_mut();
        dealer.bet = 0;
        dealer.cards = vec![];

        let players = self.state.get_players_mut();
        players.clear();

        for _ in 0..self.config.player_count {
            let player = BlackjackPlayer {
                id: Uuid::new_v4(),
                hands: vec![],
                credits: self.config.starting_credits,
                plays_with_credits: false,
                bet: None,
                has_insurance: None,
            };

            players.push(player);
        }
    }

    pub fn set_player_plays_with_credits(&mut self, player_id: Uuid, uses_credits: bool) -> BlackGameResult<()> {
        let player = self.find_player_by_id(player_id)?;
        player.plays_with_credits = uses_credits;

        Ok(())
    }

    fn assert_in_game_status(&self, statuses: Vec<BlackjackGameStatus>) -> BlackGameResult<()> {
        let current_status = self.state.get_status();
        let in_status = statuses.iter().any(|stat| stat == current_status);

        if !in_status {
            Err(BlackjackGameError::InvalidActionForGameStatus(current_status.clone()))
        } else {
            Ok(())
        }

    }

    fn find_player_by_id(&mut self, player_id: Uuid) -> BlackGameResult<&mut BlackjackPlayer> {
        let players = self.state.get_players_mut();

        players.iter_mut()
            .find(|pl| pl.id == player_id)
            .ok_or_else(|| BlackjackGameError::PlayerWithIdNotFound)
    }

    fn deal_cards(&mut self) -> BlackGameResult<()> {
        let dealer = self.state.get_dealer_hand_mut();
        dealer.cards.push(self.draw_card());
        dealer.cards.push(self.draw_card());

        for player in self.state.get_players_mut().iter_mut() {
            player.hands.push(BlackjackHand {
                cards: vec![self.draw_card(), self.draw_card()],
            });
        }

        Ok(())
    }

    fn get_first_in_row_player_id(&self) -> Option<Uuid> {
        self.state.get_players()
            .iter()
            .find(|pl| pl.bet.is_some())
            .map(|pl| pl.id)
    }

    fn handle_betting_over(&mut self) {
        self.deal_cards();

        if self.state.get_dealer_hand_mut().could_have_blackjack() {
            self.state.set_status(BlackjackGameStatus::InsuranceBetting(players.get(0).unwrap().id))
        } else {
            self.state.set_status(BlackjackGameStatus::Playing(self.get_first_in_row_player_id().unwrap()));
        }
    }

    fn handle_insurance_betting_over(&mut self) {
        let mut players_who_can_double_down = self.state.get_players_mut()
            .iter()
            .filter(|pl | pl.can_double_down());

        if let Some(player) = players_who_can_double_down.next() {
            self.state.set_status(BlackjackGameStatus::DoubleDown(player.id))
        } else {
            self.state.set_status(BlackjackGameStatus::Playing(self.get_first_in_row_player_id().unwrap()));
        }
    }

    fn handle_bet_action(&mut self, bet: BetAction) -> BlackGameResult<()> {
        let player = self.find_player_by_id(bet.player_id)?;
        if player.plays_with_credits {
            player.assert_has_more_credits_then(bet.bet)?;
            player.credits -= bet.bet;
        }
        player.bet = Some(bet.bet);

        let players = self.state.get_players_mut();
        if players.iter().all(|pp| pp.bet.is_some()) {
            self.handle_betting_over();
        }

        Ok(())
    }

    fn handle_insurance_bet_action(&mut self, insurance_bet: InsuranceBetAction) -> BlackGameResult<()> {
        let player = self.find_player_by_id(insurance_bet.player_id)?;
        if player.bet.is_none() {
            return Err(BlackjackGameError::PlayerNotPartOfRound);
        }

        if !insurance_bet.place_insurance_bet {
            player.has_insurance = Some(false)
            return Ok(());
        }

        let bet = player.bet.unwrap();
        let insurance_bet = (bet - (bet % 2)) / 2;
        player.assert_has_more_credits_then(insurance_bet)?;
        player.has_insurance = Some(true);
        player.credits -= insurance_bet;

        let players = self.state.get_players_mut();
        if players.iter().all(|pp| pp.has_insurance.is_some()) {
            self.handle_insurance_betting_over();
        }

        Ok(())
    }

    pub fn apply_action(&mut self, action: BlackjackGameAction) -> BlackGameResult<()> {
        match action {
            BlackjackGameAction::Bet(bet_action) => {
                self.assert_in_game_status(vec![BlackjackGameStatus::Betting])?;
                self.handle_bet_action(bet_action)?;

                Ok(())
            },
            BlackjackGameAction::InsuranceBet(insurance_bet) => {
                self.assert_in_game_status(vec![BlackjackGameStatus::InsuranceBetting(insurance_bet.player_id)])?;
                self.handle_insurance_bet_action(insurance_bet)?;

                Ok(())
            },
            BlackjackGameAction::DoubleDown(double_down) => {
                self.assert_in_game_status(vec![BlackjackGameStatus::DoubleDown(double_down.player_id)])?;

                Ok(())
            },
            _ => {
                todo!();
            }
        }
    }
}