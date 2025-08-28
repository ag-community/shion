use tracing::Level;

use crate::common::env::FromEnv;
use std::env;
use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Duration;

pub struct AppSettings {
    pub app_component: String,
    pub app_port: u16,

    pub log_level: Level,

    pub database_url: String,
    pub database_wait_timeout: Duration,
    pub database_max_connections: usize,

    pub steam_api_key: String,
}

impl AppSettings {
    pub fn load_from_env() -> anyhow::Result<Self> {
        let _ = dotenv::dotenv();

        let app_component = env::var("APP_COMPONENT")?;
        let app_port = u16::from_env("APP_PORT")?;

        let log_level = Level::from_env("LOG_LEVEL")?;

        let database_url = env::var("DATABASE_URL")?;
        let database_wait_timeout_secs = u64::from_env("DATABASE_WAIT_TIMEOUT_SECS")?;
        let database_wait_timeout = Duration::from_secs(database_wait_timeout_secs);
        let database_max_connections = usize::from_env("DATABASE_MAX_CONNECTIONS")?;

        let steam_api_key = env::var("STEAM_API_KEY")?;

        Ok(AppSettings {
            app_component,
            app_port,

            log_level,

            database_url,
            database_wait_timeout,
            database_max_connections,

            steam_api_key,
        })
    }

    pub fn get() -> &'static AppSettings {
        settings()
    }
}

pub fn settings() -> &'static AppSettings {
    static SETTINGS: LazyLock<AppSettings> =
        LazyLock::new(|| AppSettings::load_from_env().expect("Failed to load settings"));
    SETTINGS.deref()
}
