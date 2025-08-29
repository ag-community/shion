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
