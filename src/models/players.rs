use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    entities::players::{Player as PlayerEntity, PlayerHistory as PlayerHistoryEntity},
    models::stats::Stats,
};

#[derive(Serialize)]
pub struct Player {
    pub id: u64,
    pub steam_id: String,
    pub steam_name: String,
    pub steam_avatar_url: String,
    pub stats: Stats,
}

#[derive(Serialize)]
pub struct PlayerHistory {
    pub captures: Vec<PlayerHistoryCapture>,
}

#[derive(Serialize)]
pub struct PlayerHistoryCapture {
    pub captured_at: DateTime<Utc>,
    pub rating: f64,
}

impl From<PlayerEntity> for Player {
    fn from(value: PlayerEntity) -> Self {
        Self {
            id: value.id,
            steam_id: value.steam_id,
            steam_name: value.steam_name,
            steam_avatar_url: value.steam_avatar_url,
            stats: value.stats.into(),
        }
    }
}

impl From<PlayerHistoryEntity> for PlayerHistory {
    fn from(value: PlayerHistoryEntity) -> Self {
        Self {
            captures: value
                .captures
                .into_iter()
                .map(|capture| PlayerHistoryCapture {
                    captured_at: capture.captured_at,
                    rating: capture.rating,
                })
                .collect(),
        }
    }
}
