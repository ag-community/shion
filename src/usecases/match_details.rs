use actix_web::web::Json;
use skillratings::{
    Outcomes,
    weng_lin::{WengLinConfig, WengLinRating, weng_lin_two_teams},
};

use crate::{
    api::match_details::RequestBody,
    common::state::DbConnection,
    models::match_details::MatchDetail,
    repositories::{
        match_details,
        players::{self, PlayerId, PlayerIds},
    },
};

#[derive(Debug)]
pub enum TeamValidationError {
    InvalidModel(String),
    UnevenTeams(usize, usize),
}

pub struct PlayerRating {
    rating: WengLinRating,
    detail: MatchDetail,
}

impl std::fmt::Display for TeamValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidModel(model) => write!(
                f,
                "Invalid model value: '{}'. Valid values are 'blue' or 'red'",
                model
            ),
            Self::UnevenTeams(blue, red) => write!(
                f,
                "Team sizes do not match: blue team size = {}, red team size = {}",
                blue, red
            ),
        }
    }
}

pub fn validate_teams(details: &Json<Vec<RequestBody>>) -> Result<(), TeamValidationError> {
    let mut blue_team_size = 0;
    let mut red_team_size = 0;

    for detail in details.iter() {
        match detail.model.to_lowercase().as_str() {
            "blue" => blue_team_size += 1,
            "red" => red_team_size += 1,
            _ => return Err(TeamValidationError::InvalidModel(detail.model.to_string())),
        }
    }

    if blue_team_size != red_team_size {
        return Err(TeamValidationError::UnevenTeams(
            blue_team_size,
            red_team_size,
        ));
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

pub async fn process_match<T: DbConnection>(
    state: &T,
    match_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let match_details = match_details::fetch_match_details(state, match_id).await?;

    let player_ids: Vec<u64> = match_details
        .iter()
        .map(|detail| detail.player_id)
        .collect();
    let players = players::fetch_many(state, PlayerIds::Ids(player_ids)).await?;

    let mut player_ratings: Vec<PlayerRating> = Vec::new();

    for player in &players {
        if let Some(detail) = match_details.iter().find(|d| d.player_id == player.id) {
            player_ratings.push(PlayerRating {
                rating: WengLinRating {
                    rating: player.rating,
                    uncertainty: player.uncertainty,
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

        match players::update_player_rating(
            state,
            PlayerId::Id(detail.player_id),
            new_rating.rating,
            new_rating.uncertainty,
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to update player ratings: {}", e).into());
            }
        }

        match match_details::update_ratings(state, detail.id, new_rating.rating, *rating_delta)
            .await
        {
            Ok(_) => (),
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to update match details: {}", e).into());
            }
        }
    }

    Ok(())
}
