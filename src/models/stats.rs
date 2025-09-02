use serde::Serialize;

use crate::entities::stats::Stats as StatsEntity;

#[derive(Serialize)]
pub struct Stats {
    pub player_id: u64,
    pub rating: f64,
    pub uncertainty: f64,
    pub wins: u32,
    pub losses: u32,
    pub total_frags: i32,
    pub total_deaths: i32,
}

impl From<StatsEntity> for Stats {
    fn from(value: StatsEntity) -> Self {
        Self {
            player_id: value.player_id,
            rating: value.rating,
            uncertainty: value.uncertainty,
            wins: value.wins,
            losses: value.losses,
            total_frags: value.total_frags,
            total_deaths: value.total_deaths,
        }
    }
}
