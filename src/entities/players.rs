use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub steam_id: String,
    pub steam_name: String,
    pub steam_avatar_url: String,
    pub rating: f64,
    pub uncertainty: f64,
}
