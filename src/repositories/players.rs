use anyhow::Ok;

use crate::{common::state::DbConnection, models::players::Player};

pub enum PlayerId {
    SteamId(String),
    Id(u64),
}

pub enum PlayerIds {
    SteamIds(Vec<String>),
    Ids(Vec<u64>),
}

pub async fn create<T: DbConnection>(
    state: &T,
    steam_id: String,
    steam_name: String,
    steam_avatar_url: String,
) -> anyhow::Result<Player> {
    let player = sqlx::query_as::<_, Player>(
        "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE steam_id = ?",
    )
    .bind(&steam_id)
    .fetch_optional(state.get_connection())
    .await?;

    if let Some(existing_player) = player {
        return Ok(existing_player);
    }

    let player_id = sqlx::query(
        "INSERT INTO `player` (steam_id, steam_name, steam_avatar_url) VALUES (?, ?, ?)",
    )
    .bind(&steam_id)
    .bind(&steam_name)
    .bind(&steam_avatar_url)
    .execute(state.get_connection())
    .await?
    .last_insert_id();

    let new_player = sqlx::query_as::<_, Player>(
        "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE id = ?",
    )
    .bind(player_id)
    .fetch_one(state.get_connection())
    .await?;

    Ok(new_player)
}

pub async fn fetch_one<T: DbConnection>(state: &T, id: PlayerId) -> anyhow::Result<Player> {
    match id {
        PlayerId::Id(player_id) => {
            let player = sqlx::query_as::<_, Player>(
                "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE id = ?",
            )
            .bind(player_id)
            .fetch_one(state.get_connection())
            .await?;

            Ok(player)
        }
        PlayerId::SteamId(steam_id) => {
            let player = sqlx::query_as::<_, Player>(
                "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE steam_id = ?",
            )
            .bind(steam_id)
            .fetch_one(state.get_connection())
            .await?;

            Ok(player)
        }
    }
}

pub async fn fetch_many<T: DbConnection>(state: &T, ids: PlayerIds) -> anyhow::Result<Vec<Player>> {
    match ids {
        PlayerIds::SteamIds(steam_ids) => {
            if steam_ids.is_empty() {
                return Ok(vec![]);
            }

            let placeholders = steam_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            let query = format!(
                "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE steam_id IN ({})",
                placeholders
            );

            let mut sql_query = sqlx::query_as::<_, Player>(&query);
            for id in &steam_ids {
                sql_query = sql_query.bind(id);
            }

            let players = sql_query.fetch_all(state.get_connection()).await?;

            Ok(players)
        }
        PlayerIds::Ids(ids) => {
            if ids.is_empty() {
                return Ok(vec![]);
            }

            let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            let query = format!(
                "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player WHERE id IN ({})",
                placeholders
            );

            let mut sql_query = sqlx::query_as::<_, Player>(&query);
            for id in &ids {
                sql_query = sql_query.bind(id);
            }

            let players = sql_query.fetch_all(state.get_connection()).await?;

            Ok(players)
        }
    }
}

pub async fn fetch_leaderboard<T: DbConnection>(
    state: &T,
    page: u32,
    limit: u32,
) -> anyhow::Result<Vec<Player>> {
    let limit = std::cmp::min(limit, 50);
    let offset = (page - 1) * limit;

    let players = sqlx::query_as::<_, Player>(
        "SELECT id, steam_id, steam_name, steam_avatar_url, rating, uncertainty FROM player ORDER BY rating DESC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(state.get_connection())
    .await?;

    Ok(players)
}

pub async fn update_player_rating<T: DbConnection>(
    state: &T,
    id: PlayerId,
    rating: f64,
    uncertainty: f64,
) -> anyhow::Result<()> {
    match id {
        PlayerId::Id(player_id) => {
            sqlx::query("UPDATE `player` SET rating = ?, uncertainty = ? WHERE id = ?")
                .bind(rating)
                .bind(uncertainty)
                .bind(player_id)
                .execute(state.get_connection())
                .await?;
            Ok(())
        }
        PlayerId::SteamId(steam_id) => {
            sqlx::query("UPDATE `player` SET rating = ?, uncertainty = ? WHERE steam_id = ?")
                .bind(rating)
                .bind(uncertainty)
                .bind(steam_id)
                .execute(state.get_connection())
                .await?;
            Ok(())
        }
    }
}

pub async fn reset_all_player_ratings<T: DbConnection>(state: &T) -> anyhow::Result<()> {
    sqlx::query("UPDATE `player` SET rating = 1000, uncertainty = 333.33333")
        .execute(state.get_connection())
        .await?;
    Ok(())
}
