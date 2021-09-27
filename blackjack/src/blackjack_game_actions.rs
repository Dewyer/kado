use uuid::Uuid;

pub struct BetAction {
    pub player_id: Uuid,
    pub bet: usize,
}

pub struct InsuranceBetAction {
    pub player_id: Uuid,
    pub place_insurance_bet: bool,
}

pub struct DoubleDownAction {
    pub player_id: Uuid,
    pub doubles_down: bool,
}

pub struct PlayAction {
    pub player_id: Uuid,
    pub stay: bool,
}

#[non_exhaustive]
pub enum BlackjackGameAction {
    Bet(BetAction),
    InsuranceBet(InsuranceBetAction),
    DoubleDown(DoubleDownAction),
    Play(PlayAction),
}
