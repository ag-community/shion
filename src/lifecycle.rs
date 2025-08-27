use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

use crate::{common::state::AppState, settings::AppSettings};

pub async fn initialize_state(settings: &AppSettings) -> anyhow::Result<AppState> {
    println!("Initializing application state...");
    println!("Connecting to the database...");
    let db = initialize_db(&settings)
        .await
        .expect("Failed to connect to the database");
    Ok(AppState { db })
}

pub fn initialize_db(settings: &AppSettings) -> impl Future<Output = sqlx::Result<Pool<MySql>>> {
    MySqlPoolOptions::new()
        .acquire_timeout(settings.database_wait_timeout)
        .max_connections(settings.database_max_connections as _)
        .connect(&settings.database_url)
}
