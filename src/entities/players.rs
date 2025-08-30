use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::entities::stats::Stats;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub steam_id: String,
    pub steam_name: String,
    pub steam_avatar_url: String,
    #[sqlx(skip)]
    pub stats: Stats,
}
#[derive(Serialize, FromRow, Debug, Clone)]
pub struct PlayerHistory {
    pub captures: Vec<PlayerHistoryCapture>,
}

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct PlayerHistoryCapture {
    pub captured_at: DateTime<Utc>,
    pub rating: f64,
}
