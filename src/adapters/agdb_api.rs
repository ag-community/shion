use crate::common::error::{AppError, ServiceResult};
use reqwest::StatusCode;
use serde::Deserialize;
use std::{fmt::Display, sync::LazyLock};

fn make_url<T: Display>(steam_id: T) -> String {
    format!("https://agdb.7mochi.ru/players/{steam_id}")
}

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[derive(Debug, Deserialize)]
pub struct AGDBPlayer {
    #[serde(rename = "steamName")]
    pub steam_name: String,
    #[serde(rename = "steamID")]
    pub steam_id: String,
    #[serde(rename = "steamUrl")]
    pub steam_url: String,
    pub country: String,
}

pub async fn fetch_player_info(steam_id: String) -> ServiceResult<AGDBPlayer> {
    let url = make_url(steam_id);
    let response = CLIENT.get(url).send().await?;
    match response.status() {
        StatusCode::BAD_REQUEST => Err(AppError::AGDBInvalidSteamID),
        StatusCode::NOT_FOUND => Err(AppError::AGDBPlayerNotFound),
        _ => match response.json::<AGDBPlayer>().await {
            Err(_e) => Err(AppError::AGDBPartialData),
            Ok(player) => Ok(player),
        },
    }
}
