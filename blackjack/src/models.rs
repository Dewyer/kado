use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BlackjackGameStatus {
    Betting,
    InsuranceBetting(Uuid),
    DoubleDown(Uuid),
    Playing(Uuid),
}

#[derive(Clone)]
pub struct BlackjackGameConfig {
    pub deck_count: usize,
    pub starting_credits: usize,
    pub seed: i64,
    pub player_count: usize,
}