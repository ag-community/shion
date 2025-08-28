use crate::{
    common::state::DatabaseState,
    entities::{match_details::MatchDetail, matches::MatchDetailExtended},
};

const TABLE_NAME: &str = "match_detail";

pub async fn create<T: DatabaseState>(
    state: &T,
    steam_id: String,
    match_id: u64,
    frags: i16,
    deaths: i16,
    average_ping: u16,
    damage_dealt: u16,
    damage_taken: u16,
    model: String,
    rating_after_match: f64,
    rating_delta: f64,
) -> sqlx::Result<()> {
    const PLAYER_SELECT_QUERY: &str = "SELECT id FROM `player` WHERE steam_id = ?";
    const INSERT_QUERY: &str = const_str::concat!(
        "INSERT INTO `",
        TABLE_NAME,
        "` (player_id, match_id, frags, deaths, average_ping, damage_dealt, damage_taken, model, rating_after_match, rating_delta) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );

    let (player_id,) = sqlx::query_as::<_, (u64,)>(PLAYER_SELECT_QUERY)
        .bind(steam_id)
        .fetch_one(state.db())
        .await?;

    sqlx::query(INSERT_QUERY)
        .bind(player_id)
        .bind(match_id)
        .bind(frags)
        .bind(deaths)
        .bind(average_ping)
        .bind(damage_dealt)
        .bind(damage_taken)
        .bind(model.to_lowercase())
        .bind(rating_after_match)
        .bind(rating_delta)
        .execute(state.db())
        .await?;

    Ok(())
}

pub async fn fetch_match_details<T: DatabaseState>(
    state: &T,
    match_id: u64,
) -> sqlx::Result<Vec<MatchDetail>> {
    const QUERY: &str = const_str::concat!(
        "SELECT id, player_id, match_id, frags, deaths, average_ping, damage_dealt, damage_taken, model, rating_after_match, rating_delta ",
        "FROM `",
        TABLE_NAME,
        "` WHERE match_id = ?"
    );

    sqlx::query_as::<_, MatchDetail>(QUERY)
        .bind(match_id)
        .fetch_all(state.db())
        .await
}

pub async fn fetch_extended_match_details<T: DatabaseState>(
    state: &T,
    match_id: u64,
) -> sqlx::Result<Vec<MatchDetailExtended>> {
    sqlx::query_as::<_, MatchDetailExtended>(
        "SELECT m.id, m.player_id, s.steam_name, s.steam_id, s.steam_avatar_url, m.frags, m.deaths, m.average_ping, m.damage_dealt, m.damage_taken, m.model, m.rating_after_match, m.rating_delta
         FROM match_detail m
         LEFT JOIN player s ON m.player_id = s.id
         WHERE m.match_id = ?", 
    )
    .bind(match_id)
    .fetch_all(state.db())
    .await
}

pub async fn update_ratings<T: DatabaseState>(
    state: &T,
    id: u64,
    rating_after_match: f64,
    rating_delta: f64,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE `match_detail` SET rating_after_match = ?, rating_delta = ? WHERE id = ?")
        .bind(rating_after_match)
        .bind(rating_delta)
        .bind(id)
        .execute(state.db())
        .await?;

    Ok(())
}
