use steam_api_client::{Player as SteamPlayer, SteamClient};
use steamid_ng::SteamID;

use crate::{
    common::{
        error::{AppError, ServiceResult, unexpected},
        state::DatabaseState,
    },
    entities::{matches::MatchExtended, players::Player},
    repositories::{
        matches,
        players::{self},
        stats,
    },
    settings::AppSettings,
};

pub async fn create_player<T: DatabaseState>(state: &T, steam_id: String) -> ServiceResult<Player> {
    let settings = AppSettings::get();
    let steam_id_2 = match SteamID::from_steam2(steam_id.as_str()) {
        Ok(id) => id,
        Err(_e) => {
            return Err(AppError::PlayerSteamIDInvalid);
        }
    };
    let client = SteamClient::new(settings.steam_api_key.to_string());

    let steam_response = client
        .get_player_summaries(vec![u64::from(steam_id_2).to_string()])
        .await?;

    let player_steam_info: &SteamPlayer = match steam_response.playersummaries.players.get(0) {
        Some(player) => player,
        None => {
            return Err(AppError::PlayerSteamDoesNotExist);
        }
    };

    let player = match players::fetch_one_by_steamid(state, steam_id.to_string()).await {
        Ok(player) => player,
        Err(_e) => {
            return Ok(players::create(
                state,
                steam_id.to_string(),
                player_steam_info.personaname.to_string(),
                player_steam_info.avatarfull.to_string(),
            )
            .await?);
        }
    };
    Ok(player)
}

pub async fn fetch_player<T: DatabaseState>(state: &T, id: u64) -> ServiceResult<Player> {
    match players::fetch_one_by_id(state, id).await {
        Ok(mut player) => {
            let stats = stats::fetch_one_by_player_id(state, id).await?;
            player.stats = stats;

            Ok(player)
        }
        Err(sqlx::Error::RowNotFound) => Err(AppError::PlayerNotFound),
        Err(e) => unexpected(e),
    }
}

pub async fn fetch_player_matches<T: DatabaseState>(
    state: &T,
    id: u64,
    page: u32,
    limit: u32,
) -> ServiceResult<Vec<MatchExtended>> {
    let mut existing_matches = match matches::fetch_extended_matches(state, id, page, limit).await {
        Ok(matches) => matches,
        Err(sqlx::Error::RowNotFound) => return Err(AppError::PlayerMatchesNotFound),
        Err(e) => return unexpected(e),
    };

    for existing_match in &mut existing_matches {
        let existing_match_details =
            crate::repositories::match_details::fetch_extended_match_details(
                state,
                existing_match.id,
            )
            .await?;

        existing_match.match_details = existing_match_details;
    }
    Ok(existing_matches)
}

pub async fn fetch_leaderboard<T: DatabaseState>(
    state: &T,
    page: u32,
    limit: u32,
) -> ServiceResult<Vec<Player>> {
    let mut leaderboard = players::fetch_leaderboard(state, page, limit).await?;
    for player in &mut leaderboard {
        let stats = stats::fetch_one_by_player_id(state, player.id).await?;

        player.stats = stats;
    }
    Ok(leaderboard)
}
