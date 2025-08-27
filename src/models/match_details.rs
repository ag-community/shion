use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct MatchDetail {
    pub id: u64,
    pub player_id: u64,
    pub match_id: u64,
    pub frags: i16,
    pub deaths: i16,
    pub average_ping: u16,
    pub damage_dealt: u16,
    pub damage_taken: u16,
    pub model: String,
    pub rating_after_match: f64,
    pub rating_delta: f64,
}
