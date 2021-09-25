use uuid::Uuid;

pub struct BetAction {
    pub player_id: Uuid,
    pub bet: usize,
}

#[non_exhaustive]
pub enum BlackjackGameAction {
    Deal,
    Bet(BetAction),
}
