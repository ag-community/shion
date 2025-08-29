use crate::{common::state::DatabaseState, entities::stats::Stats};

const TABLE_NAME: &str = "stats";

pub async fn fetch_one_by_player_id<T: DatabaseState>(
    state: &T,
    player_id: u64,
) -> sqlx::Result<Stats> {
    const QUERY: &str = const_str::concat!(
        "SELECT player_id, rating, uncertainty, wins, losses, total_frags, total_deaths FROM `",
        TABLE_NAME,
        "` WHERE player_id = ?"
    );

    sqlx::query_as::<_, Stats>(QUERY)
        .bind(player_id)
        .fetch_one(state.db())
        .await
}

pub async fn update_stats<T: DatabaseState>(
    state: &T,
    player_id: u64,
    rating: f64,
    uncertainty: f64,
    wins: u32,
    losses: u32,
    total_frags: i32,
    total_deaths: i32,
) -> sqlx::Result<()> {
    const QUERY: &str = const_str::concat!(
        "INSERT INTO `",
        TABLE_NAME,
        "` (player_id, rating, uncertainty, wins, losses, total_frags, total_deaths) ",
        "VALUES (?, ?, ?, ?, ?, ?, ?) ",
        "ON DUPLICATE KEY UPDATE ",
        "rating = ?, ",
        "uncertainty = ?, ",
        "wins = wins + ?, ",
        "losses = losses + ?, ",
        "total_frags = total_frags + ?, ",
        "total_deaths = total_deaths + ?"
    );

    sqlx::query(QUERY)
        .bind(player_id)
        .bind(rating)
        .bind(uncertainty)
        .bind(wins)
        .bind(losses)
        .bind(total_frags)
        .bind(total_deaths)
        .bind(rating)
        .bind(uncertainty)
        .bind(wins)
        .bind(losses)
        .bind(total_frags)
        .bind(total_deaths)
        .execute(state.db())
        .await
        .map(|_| ())
}

pub async fn reset_all_player_stats<T: DatabaseState>(state: &T) -> sqlx::Result<()> {
    const QUERY: &str = const_str::concat!(
        "UPDATE `",
        TABLE_NAME,
        "` SET rating = 1000, uncertainty = 333.33333, wins = 0, losses = 0, total_frags = 0, total_deaths = 0"
    );

    sqlx::query(QUERY).execute(state.db()).await?;
    Ok(())
}
