use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

use crate::{common::state::State, settings::AppSettings};

pub fn initialize_logging(settings: &AppSettings) {
    tracing_subscriber::fmt()
        .with_max_level(settings.log_level)
        .json()
        .with_timer(tracing_subscriber::fmt::time())
        .with_level(true)
        .init();
}

pub async fn initialize_state(settings: &AppSettings) -> anyhow::Result<State> {
    info!("Initializing application state...");
    info!("Connecting to the database...");
    let db = initialize_db(settings)
        .await
        .expect("Failed to connect to the database");
    Ok(State { db })
}

pub fn initialize_db(settings: &AppSettings) -> impl Future<Output = sqlx::Result<Pool<MySql>>> {
    MySqlPoolOptions::new()
        .acquire_timeout(settings.database_wait_timeout)
        .max_connections(settings.database_max_connections as _)
        .connect(&settings.database_url)
}
