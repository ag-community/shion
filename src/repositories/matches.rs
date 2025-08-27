use anyhow::Ok;

use crate::{
    common::state::DbConnection,
    models::matches::{Match, MatchExtended},
};

pub enum PlayerId {
    SteamId(String),
    Id(u64),
}

pub async fn create<T: DbConnection>(
    state: &T,
    server_ip: String,
    map_name: String,
) -> anyhow::Result<Match> {
    let match_id = sqlx::query("INSERT INTO `match` (server_ip, map_name) VALUES (?, ?)")
        .bind(server_ip)
        .bind(map_name)
        .execute(state.get_connection())
        .await?
        .last_insert_id();

    let new_match =
        sqlx::query_as::<_, Match>("SELECT id, server_ip, map_name FROM `match` WHERE id = ?")
            .bind(match_id)
            .fetch_one(state.get_connection())
            .await?;

    Ok(new_match)
}

pub async fn fetch_extended_matches<T: DbConnection>(
    state: &T,
    id: u64,
) -> anyhow::Result<Vec<MatchExtended>> {
    let matches = sqlx::query_as::<_, MatchExtended>(
        "SELECT m.id, m.server_ip, m.match_date, m.map_name, md.rating_after_match, md.rating_delta
         FROM `match` m
         JOIN match_detail md ON m.id = md.match_id
         WHERE md.player_id = ?
         ORDER BY m.id DESC",
    )
    .bind(id)
    .fetch_all(state.get_connection())
    .await?;

    Ok(matches)
}

pub async fn fetch_extended_match<T: DbConnection>(
    state: &T,
    id: u64,
) -> anyhow::Result<MatchExtended> {
    let a_match = sqlx::query_as::<_, MatchExtended>(
        "SELECT m.id, m.server_ip, m.match_date, m.map_name, md.frags, md.deaths, md.rating_after_match, md.rating_delta
         FROM `match` m
         JOIN match_detail md ON m.id = md.match_id
         WHERE m.id = ?",
    )
    .bind(id)
    .fetch_one(state.get_connection())
    .await?;

    Ok(a_match)
}

pub async fn fetch_all_matches<T: DbConnection>(state: &T) -> anyhow::Result<Vec<Match>> {
    let matches = sqlx::query_as::<_, Match>("SELECT id, server_ip, map_name FROM `match`")
        .fetch_all(state.get_connection())
        .await?;

    Ok(matches)
}
