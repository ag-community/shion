use actix_web::{
    get, post,
    web::{self, Data, Json},
};
use serde::Deserialize;

use crate::{
    common::{error::ServiceResponse, state::State},
    models::matches::{Match, MatchExtended},
    usecases::matches,
};

#[derive(Deserialize)]
pub struct RequestBody {
    pub server_ip: String,
    pub map_name: String,
}

#[get("/{id}")]
async fn fetch_match(state: Data<State>, path: web::Path<u64>) -> ServiceResponse<MatchExtended> {
    let a_match = matches::fetch_match(&state, path.into_inner()).await?;
    Ok(Json(a_match))
}

#[post("/")]
async fn create_match(state: Data<State>, body: Json<RequestBody>) -> ServiceResponse<Match> {
    let new_match = matches::create_match(
        &state,
        body.map_name.to_string(),
        body.server_ip.to_string(),
    )
    .await?;
    Ok(Json(new_match))
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/matches")
        .service(fetch_match)
        .service(create_match);

    conf.service(scope);
}
