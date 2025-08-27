use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data, Json, Query},
};
use serde::Deserialize;
use steam_api_client::{Player, SteamClient};
use steamid_ng::SteamID;

use crate::{
    common::state::AppState,
    repositories::{
        matches,
        players::{self, PlayerId},
    },
    settings::AppSettings,
};

#[derive(Deserialize)]
pub struct RequestBody {
    pub steam_id: String,
}

#[derive(Deserialize)]
pub struct RequestQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

#[get("/leaderboard")]
async fn fetch_leaderboard(state: Data<AppState>, query: Query<RequestQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    match players::fetch_leaderboard(&state, page, limit).await {
        Ok(leaderboard) => HttpResponse::Ok().json(leaderboard),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to fetch leaderboard: {}", e))
        }
    }
}

#[get("/{id}/matches")]
async fn fetch_player_matches(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let id = match id_str.parse::<u64>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            return HttpResponse::BadRequest().json("ID must be a valid integer");
        }
    };

    let mut existing_matches = match matches::fetch_extended_matches(&state, id).await {
        Ok(matches) => matches,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to fetch matches: {}", e));
        }
    };

    for existing_match in &mut existing_matches {
        let existing_match_details =
            match crate::repositories::match_details::fetch_extended_match_details(
                &state,
                existing_match.id,
            )
            .await
            {
                Ok(details) => details,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(format!("Failed to fetch match details: {}", e));
                }
            };
        existing_match.match_details = existing_match_details;
    }

    HttpResponse::Ok().json(existing_matches)
}

#[get("/{id}")]
async fn fetch_player(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let id = match id_str.parse::<u64>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            return HttpResponse::BadRequest().json("ID must be a valid integer");
        }
    };

    match players::fetch_one(&state, PlayerId::Id(id)).await {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to fetch player: {}", e))
        }
    }
}

#[post("/")]
async fn create_player(state: Data<AppState>, body: Json<RequestBody>) -> impl Responder {
    let settings = AppSettings::get();
    let steam_id = match SteamID::from_steam2(body.steam_id.as_str()) {
        Ok(id) => id,
        Err(_e) => {
            return HttpResponse::BadRequest().json("Invalid Steam ID format");
        }
    };
    let client = SteamClient::new(settings.steam_api_key.to_string());

    let steam_response = match client
        .get_player_summaries(vec![u64::from(steam_id).to_string()])
        .await
    {
        Ok(response) => response,
        Err(_e) => {
            // TODO: Don't throw error, create a player with '' as name and avatar
            return HttpResponse::InternalServerError().json("Failed to fetch player summaries");
        }
    };

    let player_steam_info: &Player = match steam_response.playersummaries.players.get(0) {
        Some(player) => player,
        None => {
            return HttpResponse::NotFound().json("No player found for the given Steam ID");
        }
    };

    match players::create(
        &state,
        body.steam_id.to_string(),
        player_steam_info.personaname.to_string(),
        player_steam_info.avatarfull.to_string(),
    )
    .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to create player: {}", e));
        }
    }
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/players")
        .service(fetch_leaderboard)
        .service(fetch_player_matches)
        .service(fetch_player)
        .service(create_player);

    conf.service(scope);
}
