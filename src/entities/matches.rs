use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Match {
    pub id: u64,
    pub server_ip: String,
    pub map_name: String,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct MatchExtended {
    pub id: u64,
    pub server_ip: String,
    pub match_date: DateTime<Utc>,
    pub map_name: String,
    #[sqlx(skip)]
    pub match_details: Vec<MatchDetailExtended>,
    pub rating_after_match: f64,
    pub rating_delta: f64,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct MatchDetailExtended {
    pub id: u64,
    pub player_id: Option<u64>,
    pub steam_name: Option<String>,
    pub steam_id: Option<String>,
    pub steam_avatar_url: Option<String>,
    pub frags: i16,
    pub deaths: i16,
    pub average_ping: Option<u16>,
    pub damage_dealt: Option<u16>,
    pub damage_taken: Option<u16>,
    pub model: Option<String>,
    pub rating_after_match: Option<f64>,
    pub rating_delta: Option<f64>,
}
