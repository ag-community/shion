use crate::{common::state::DatabaseState, entities::matches::Match};

const TABLE_NAME: &str = "match";

pub async fn create<T: DatabaseState>(
    state: &T,
    server_ip: String,
    map_name: String,
) -> sqlx::Result<Match> {
    const INSERT_QUERY: &str = const_str::concat!(
        "INSERT INTO `",
        TABLE_NAME,
        "` (server_ip, map_name) VALUES (?, ?)"
    );
    const SELECT_QUERY: &str = const_str::concat!(
        "SELECT id, server_ip, map_name FROM `",
        TABLE_NAME,
        "` WHERE id = ?"
    );

    let match_id = sqlx::query(INSERT_QUERY)
        .bind(server_ip)
        .bind(map_name)
        .execute(state.db())
        .await?
        .last_insert_id();

    sqlx::query_as::<_, Match>(SELECT_QUERY)
        .bind(match_id)
        .fetch_one(state.db())
        .await
}

pub async fn fetch_matches<T: DatabaseState>(
    state: &T,
    id: u64,
    page: u32,
    limit: u32,
) -> sqlx::Result<Vec<Match>> {
    const QUERY: &str = const_str::concat!(
        "SELECT m.id, m.server_ip, m.match_date, m.map_name, md.rating_after_match, md.rating_delta ",
        "FROM `",
        TABLE_NAME,
        "` m ",
        "JOIN match_detail md ON m.id = md.match_id ",
        "WHERE md.player_id = ? ",
        "ORDER BY m.id DESC LIMIT ? OFFSET ?"
    );
    let limit = std::cmp::min(limit, 50);
    let offset = (page - 1) * limit;

    sqlx::query_as::<_, Match>(QUERY)
        .bind(id)
        .bind(limit)
        .bind(offset)
        .fetch_all(state.db())
        .await
}

pub async fn fetch_match<T: DatabaseState>(state: &T, id: u64) -> sqlx::Result<Match> {
    const QUERY: &str = const_str::concat!(
        "SELECT m.id, m.server_ip, m.match_date, m.map_name, md.frags, md.deaths, md.rating_after_match, md.rating_delta ",
        "FROM `",
        TABLE_NAME,
        "` m ",
        "JOIN match_detail md ON m.id = md.match_id ",
        "WHERE m.id = ?"
    );

    sqlx::query_as::<_, Match>(QUERY)
        .bind(id)
        .fetch_one(state.db())
        .await
}

pub async fn fetch_all_matches<T: DatabaseState>(state: &T) -> sqlx::Result<Vec<Match>> {
    const QUERY: &str =
        const_str::concat!("SELECT id, server_ip, map_name FROM `", TABLE_NAME, "`");
    sqlx::query_as::<_, Match>(QUERY)
        .fetch_all(state.db())
        .await
}
