use actix_web::{
    get, post,
    web::{self, Data, Json, Query},
};
use serde::Deserialize;

use crate::{
    common::{error::ServiceResponse, state::State},
    entities::{matches::MatchExtended, players::Player},
    usecases::players,
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
async fn fetch_leaderboard(
    state: Data<State>,
    query: Query<RequestQuery>,
) -> ServiceResponse<Vec<Player>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let leaderboard = players::fetch_leaderboard(&state, page, limit).await?;
    Ok(Json(leaderboard))
}

#[get("/{id}/matches")]
async fn fetch_player_matches(
    state: Data<State>,
    path: web::Path<u64>,
    query: Query<RequestQuery>,
) -> ServiceResponse<Vec<MatchExtended>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let player_matches =
        players::fetch_player_matches(&state, path.into_inner(), page, limit).await?;
    Ok(Json(player_matches))
}

#[get("/{id}")]
async fn fetch_player(state: Data<State>, path: web::Path<u64>) -> ServiceResponse<Player> {
    let player = players::fetch_player(&state, path.into_inner()).await?;
    Ok(Json(player))
}

#[post("/")]
async fn create_player(state: Data<State>, body: Json<RequestBody>) -> ServiceResponse<Player> {
    let player = players::create_player(&state, body.steam_id.to_string()).await?;
    Ok(Json(player))
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/players")
        .service(fetch_leaderboard)
        .service(fetch_player_matches)
        .service(fetch_player)
        .service(create_player);

    conf.service(scope);
}
