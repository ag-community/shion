use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::entities::stats::Stats;

#[derive(FromRow)]
pub struct Player {
    pub id: u64,
    pub steam_id: String,
    pub steam_name: String,
    pub steam_avatar_url: String,
    pub country: String,
    #[sqlx(skip)]
    pub stats: Stats,
}

#[derive(FromRow)]
pub struct PlayerHistory {
    pub captures: Vec<PlayerHistoryCapture>,
}

#[derive(FromRow)]
pub struct PlayerHistoryCapture {
    pub captured_at: DateTime<Utc>,
    pub rating: f64,
}
