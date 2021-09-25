
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BlackjackGameStatus {
    Betting,
}

#[derive(Clone)]
pub struct BlackjackGameConfig {
    pub deck_count: usize,
    pub starting_credits: usize,
    pub seed: i64,
    pub player_count: usize,
}