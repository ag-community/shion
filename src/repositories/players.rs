use crate::{common::state::DatabaseState, entities::players::Player};

const TABLE_NAME: &str = "player";

pub async fn create<T: DatabaseState>(
    state: &T,
    steam_id: String,
    steam_name: String,
    steam_avatar_url: String,
) -> sqlx::Result<Player> {
    const INSERT_QUERY: &str = const_str::concat!(
        "INSERT INTO `",
        TABLE_NAME,
        "` (steam_id, steam_name, steam_avatar_url) VALUES (?, ?, ?)"
    );
    const SELECT_QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url FROM `",
        TABLE_NAME,
        "` WHERE id = ?"
    );

    let player_id = sqlx::query(INSERT_QUERY)
        .bind(&steam_id)
        .bind(&steam_name)
        .bind(&steam_avatar_url)
        .execute(state.db())
        .await?
        .last_insert_id();

    sqlx::query_as::<_, Player>(SELECT_QUERY)
        .bind(player_id)
        .fetch_one(state.db())
        .await
}

pub async fn fetch_one_by_id<T: DatabaseState>(state: &T, id: u64) -> sqlx::Result<Player> {
    const QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url FROM `",
        TABLE_NAME,
        "` WHERE id = ?"
    );

    sqlx::query_as::<_, Player>(QUERY)
        .bind(id)
        .fetch_one(state.db())
        .await
}

pub async fn fetch_one_by_steamid<T: DatabaseState>(
    state: &T,
    steam_id: String,
) -> sqlx::Result<Player> {
    const QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url FROM `",
        TABLE_NAME,
        "` WHERE steam_id = ?"
    );

    sqlx::query_as::<_, Player>(QUERY)
        .bind(steam_id)
        .fetch_one(state.db())
        .await
}

pub async fn fetch_many_by_ids<T: DatabaseState>(
    state: &T,
    ids: Vec<u64>,
) -> sqlx::Result<Vec<Player>> {
    let values = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!(
        "SELECT id, steam_id, steam_name, steam_avatar_url FROM {} WHERE id IN ({})",
        TABLE_NAME, values
    );

    let mut sql_query = sqlx::query_as::<_, Player>(&query);
    for id in &ids {
        sql_query = sql_query.bind(id);
    }

    sql_query.fetch_all(state.db()).await
}

pub async fn fetch_all<T: DatabaseState>(state: &T) -> sqlx::Result<Vec<Player>> {
    const QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url FROM `",
        TABLE_NAME,
        "`"
    );

    sqlx::query_as::<_, Player>(QUERY)
        .fetch_all(state.db())
        .await
}

pub async fn fetch_leaderboard<T: DatabaseState>(
    state: &T,
    page: u32,
    limit: u32,
) -> sqlx::Result<Vec<Player>> {
    const QUERY: &str = const_str::concat!(
        "SELECT p.id, p.steam_id, p.steam_name, p.steam_avatar_url FROM `",
        TABLE_NAME,
        "` p LEFT JOIN stats s ON p.id = s.player_id ORDER BY s.rating DESC LIMIT ? OFFSET ?"
    );
    let limit = std::cmp::min(limit, 50);
    let offset = (page - 1) * limit;

    sqlx::query_as::<_, Player>(QUERY)
        .bind(limit)
        .bind(offset)
        .fetch_all(state.db())
        .await
}
