use crate::{
    common::state::DatabaseState,
    entities::players::{Player, PlayerHistoryCapture},
};

const TABLE_NAME: &str = "player";

pub async fn create<T: DatabaseState>(
    state: &T,
    steam_id: String,
    steam_name: String,
    steam_avatar_url: String,
    country: String,
) -> sqlx::Result<Player> {
    const INSERT_QUERY: &str = const_str::concat!(
        "INSERT INTO `",
        TABLE_NAME,
        "` (steam_id, steam_name, steam_avatar_url, country) VALUES (?, ?, ?, ?)"
    );
    const SELECT_QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM `",
        TABLE_NAME,
        "` WHERE id = ?"
    );

    let player_id = sqlx::query(INSERT_QUERY)
        .bind(&steam_id)
        .bind(&steam_name)
        .bind(&steam_avatar_url)
        .bind(&country)
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
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM `",
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
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM `",
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
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM {} WHERE id IN ({})",
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
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM `",
        TABLE_NAME,
        "`"
    );

    sqlx::query_as::<_, Player>(QUERY)
        .fetch_all(state.db())
        .await
}

pub async fn fetch_all_with_unknown_country<T: DatabaseState>(
    state: &T,
) -> sqlx::Result<Vec<Player>> {
    const QUERY: &str = const_str::concat!(
        "SELECT id, steam_id, steam_name, steam_avatar_url, country FROM `",
        TABLE_NAME,
        "` WHERE country = 'xx'"
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
        "SELECT p.id, p.steam_id, p.steam_name, p.steam_avatar_url, p.country FROM `",
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

pub async fn update_country<T: DatabaseState>(
    state: &T,
    id: u64,
    country: String,
) -> sqlx::Result<()> {
    const QUERY: &str =
        const_str::concat!("UPDATE `", TABLE_NAME, "` SET country = ? WHERE id = ?");

    sqlx::query(QUERY)
        .bind(country)
        .bind(id)
        .execute(state.db())
        .await?;
    Ok(())
}

// TODO: Move this to matches maybe?
pub async fn fetch_rating_history<T: DatabaseState>(
    state: &T,
    id: u64,
) -> sqlx::Result<Vec<PlayerHistoryCapture>> {
    const QUERY: &str = "
        SELECT t.captured_at, t.rating
        FROM (
            SELECT 
                m.match_date as captured_at, 
                md.rating_after_match as rating,
                DATE(m.match_date) as match_day,
                ROW_NUMBER() OVER (PARTITION BY DATE(m.match_date) ORDER BY m.match_date DESC) as rn
            FROM match_detail md
            JOIN `match` m ON md.match_id = m.id
            WHERE md.player_id = ?
              AND m.match_date >= DATE_SUB(NOW(), INTERVAL 90 DAY)
        ) t
        WHERE t.rn = 1
        ORDER BY t.captured_at ASC
    ";

    sqlx::query_as::<_, PlayerHistoryCapture>(QUERY)
        .bind(id)
        .fetch_all(state.db())
        .await
}
