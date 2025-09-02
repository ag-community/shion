use sqlx::prelude::FromRow;

#[derive(FromRow, Default)]
pub struct Stats {
    pub player_id: u64,
    pub rating: f64,
    pub uncertainty: f64,
    pub wins: u32,
    pub losses: u32,
    pub total_frags: i32,
    pub total_deaths: i32,
}
