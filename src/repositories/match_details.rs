use std::result::Result::Ok;

use crate::{
    common::state::DbConnection,
    models::{match_details::MatchDetail, matches::MatchDetailExtended},
};

pub async fn create<T: DbConnection>(
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
) -> anyhow::Result<()> {
    let player_id = match sqlx::query_as::<_, (u64,)>("SELECT id FROM `player` WHERE steam_id = ?")
        .bind(steam_id)
        .fetch_one(state.get_connection())
        .await
    {
        Ok((player_id,)) => player_id,
        Err(_) => {
            return Err(anyhow::anyhow!("Player not found"));
        }
    };

    sqlx::query(
        "INSERT INTO `match_detail` (player_id, match_id, frags, deaths, average_ping, damage_dealt, damage_taken, model, rating_after_match, rating_delta) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    ).bind(player_id)
    .bind(match_id)
    .bind(frags)
    .bind(deaths)
    .bind(average_ping)
    .bind(damage_dealt)
    .bind(damage_taken)
    .bind(model.to_lowercase())
    .bind(rating_after_match)
    .bind(rating_delta)
    .execute(state.get_connection())
    .await?;

    Ok(())
}

pub async fn fetch_match_details<T: DbConnection>(
    state: &T,
    match_id: u64,
) -> anyhow::Result<Vec<MatchDetail>> {
    let match_details = sqlx::query_as::<_, MatchDetail>(
        "SELECT id, player_id, match_id, frags, deaths, average_ping, damage_dealt, damage_taken, model, rating_after_match, rating_delta FROM `match_detail` WHERE match_id = ?",
    )
    .bind(match_id)
    .fetch_all(state.get_connection())
    .await?;

    Ok(match_details)
}

pub async fn fetch_extended_match_details<T: DbConnection>(
    state: &T,
    match_id: u64,
) -> anyhow::Result<Vec<MatchDetailExtended>> {
    let match_details = sqlx::query_as::<_, MatchDetailExtended>(
        "SELECT m.id, m.player_id, s.steam_name, s.steam_id, s.steam_avatar_url, m.frags, m.deaths, m.average_ping, m.damage_dealt, m.damage_taken, m.model, m.rating_after_match, m.rating_delta
         FROM match_detail m
         LEFT JOIN player s ON m.player_id = s.id
         WHERE m.match_id = ?", 
    )
    .bind(match_id)
    .fetch_all(state.get_connection())
    .await?;

    Ok(match_details)
}

pub async fn update_ratings<T: DbConnection>(
    state: &T,
    id: u64,
    rating_after_match: f64,
    rating_delta: f64,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE `match_detail` SET rating_after_match = ?, rating_delta = ? WHERE id = ?")
        .bind(rating_after_match)
        .bind(rating_delta)
        .bind(id)
        .execute(state.get_connection())
        .await?;

    Ok(())
}
