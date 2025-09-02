use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(FromRow, Clone)]
pub struct Match {
    pub id: u64,
    pub server_ip: String,
    #[sqlx(default)]
    pub match_date: DateTime<Utc>,
    pub map_name: String,
    #[sqlx(default)]
    pub frags: i16,
    #[sqlx(default)]
    pub deaths: i16,
    #[sqlx(default)]
    pub rating_after_match: f64,
    #[sqlx(default)]
    pub rating_delta: f64,
}
