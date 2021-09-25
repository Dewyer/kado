use crate::blackjack_game_state::IBlackjackGameStateProvider;
use crate::rng::SeededRng;
use crate::errors::{BlackGameResult, BlackjackGameError};
use crate::models::{BlackjackGameStatus, BlackjackGameConfig};
use crate::blackjack_game_actions::{BlackjackGameAction, BetAction};
use crate::blackjack_player::BlackjackPlayer;
use uuid::Uuid;
use crate::blackjack_hand::BlackjackHand;

pub struct BlackjackGame {
    rng: SeededRng,
    state: IBlackjackGameStateProvider,
    config: BlackjackGameConfig,
}

impl BlackjackGame {
    pub fn new(state: IBlackjackGameStateProvider, config: BlackjackGameConfig) -> Self {
        let mut game = Self {
            state,
            rng: SeededRng::new(config.seed),
            config,
        };
        game.rng.skip_numbers(game.state.get_card_index());

        game
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
            };

            players.push(player);
        }
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

    fn handle_bet_action(&mut self, bet: BetAction) -> BlackGameResult<()> {
        let player = self.find_player_by_id(bet.player_id)?;
        if player.plays_with_credits {
            player.assert_has_more_credits_then(bet.bet)?;
            player.credits -= bet.bet;
        }

        player.hands.push(BlackjackHand::from_bet(bet.bet));

        Ok(())
    }

    pub fn apply_action(&mut self, action: BlackjackGameAction) -> BlackGameResult<()> {
        match action {
            BlackjackGameAction::Bet(bet_action) => {
                self.assert_in_game_status(vec![BlackjackGameStatus::Betting])?;
                self.handle_bet_action(bet_action)?;

                Ok(())
            },
            BlackjackGameAction::Deal => {
                todo!();
            },
            _ => {
                todo!();
            }
        }
    }
}