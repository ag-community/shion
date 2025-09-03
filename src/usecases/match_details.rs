use actix_web::web::Json;
use skillratings::{
    Outcomes,
    weng_lin::{WengLinConfig, WengLinRating, weng_lin_two_teams},
};

use crate::{
    api::match_details::RequestBody,
    common::{
        error::{AppError, ServiceResult},
        state::DatabaseState,
    },
    entities::match_details::MatchDetail,
    repositories::{
        match_details,
        players::{self},
        stats,
    },
};

pub struct PlayerRating {
    rating: WengLinRating,
    detail: MatchDetail,
}

pub fn validate_teams(details: &Json<Vec<RequestBody>>) -> ServiceResult<()> {
    let mut blue_team_size = 0;
    let mut red_team_size = 0;

    for detail in details.iter() {
        match detail.model.to_lowercase().as_str() {
            "blue" => blue_team_size += 1,
            "red" => red_team_size += 1,
            _ => return Err(AppError::InvalidModel),
        }
    }

    if blue_team_size != red_team_size {
        return Err(AppError::UnevenTeams);
    }

    Ok(())
}

pub fn determine_winner(blue_stats: &Vec<i16>, red_stats: &Vec<i16>) -> Outcomes {
    let blue_score: i16 = blue_stats.iter().sum();
    let red_score: i16 = red_stats.iter().sum();

    if blue_score > red_score {
        Outcomes::WIN
    } else {
        Outcomes::LOSS
    }
}

pub async fn create_match_details<T: DatabaseState>(
    state: &T,
    details: Json<Vec<RequestBody>>,
) -> ServiceResult<()> {
    validate_teams(&details)?;

    for detail in details.iter() {
        match_details::create(
            state,
            detail.steam_id.to_string(),
            detail.match_id,
            detail.frags,
            detail.deaths,
            detail.average_ping,
            detail.damage_dealt,
            detail.damage_taken,
            detail.model.to_string(),
            0.0,
            0.0,
        )
        .await?;
    }
    Ok(())
}

pub async fn process_match<T: DatabaseState>(state: &T, match_id: u64) -> ServiceResult<()> {
    let match_details = match_details::fetch_match_details(state, match_id).await?;

    if match_details.is_empty() {
        warn!(
            "No match details found for match ID: {}, skipping processing.",
            match_id
        );
        return Ok(());
    }

    let player_ids: Vec<u64> = match_details
        .iter()
        .map(|detail| detail.player_id)
        .collect();
    let players = players::fetch_many_by_ids(state, player_ids).await?;

    let mut player_ratings: Vec<PlayerRating> = Vec::new();

    for player in &players {
        if let Some(detail) = match_details.iter().find(|d| d.player_id == player.id) {
            let stats = stats::fetch_one_by_player_id(state, player.id).await?;

            player_ratings.push(PlayerRating {
                rating: WengLinRating {
                    rating: stats.rating,
                    uncertainty: stats.uncertainty,
                },
                detail: detail.clone(),
            });
        }
    }

    let (blue_team_players, red_team_players): (Vec<&PlayerRating>, Vec<&PlayerRating>) =
        player_ratings
            .iter()
            .partition(|pr| pr.detail.model == "blue");

    let blue_team_ratings: Vec<WengLinRating> =
        blue_team_players.iter().map(|pr| pr.rating).collect();

    let red_team_ratings: Vec<WengLinRating> =
        red_team_players.iter().map(|pr| pr.rating).collect();

    let config = WengLinConfig::new();

    let outcome = determine_winner(
        &match_details
            .iter()
            .filter(|d| d.model == "blue")
            .map(|d| d.frags)
            .collect(),
        &match_details
            .iter()
            .filter(|d| d.model == "red")
            .map(|d| d.frags)
            .collect(),
    );

    let (new_blue_ratings, new_red_ratings) =
        weng_lin_two_teams(&blue_team_ratings, &red_team_ratings, &outcome, &config);

    let mut updated_ratings = std::collections::HashMap::new();

    blue_team_players
        .iter()
        .zip(new_blue_ratings.iter())
        .for_each(|(player, &new_rating)| {
            let rating_delta = new_rating.rating - player.rating.rating;
            updated_ratings.insert(player.detail.player_id.clone(), (new_rating, rating_delta));
        });

    red_team_players
        .iter()
        .zip(new_red_ratings.iter())
        .for_each(|(player, &new_rating)| {
            let rating_delta = new_rating.rating - player.rating.rating;
            updated_ratings.insert(player.detail.player_id.clone(), (new_rating, rating_delta));
        });

    for detail in match_details.iter() {
        let (new_rating, rating_delta) = updated_ratings.get(&detail.player_id).unwrap_or(&(
            WengLinRating {
                rating: 0.0,
                uncertainty: 0.0,
            },
            0.0,
        ));

        let is_winner = (detail.model == "blue" && outcome == Outcomes::WIN)
            || (detail.model == "red" && outcome == Outcomes::LOSS);

        stats::update_stats(
            state,
            detail.player_id,
            new_rating.rating,
            new_rating.uncertainty,
            if is_winner { 1 } else { 0 },
            if !is_winner { 1 } else { 0 },
            detail.frags as i32,
            detail.deaths as i32,
        )
        .await?;

        match_details::update_ratings(state, detail.id, new_rating.rating, *rating_delta).await?;
    }

    Ok(())
}
