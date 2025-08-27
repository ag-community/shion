use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data, Json},
};
use serde::Deserialize;

use crate::{
    common::state::AppState,
    repositories::{match_details, matches},
};

#[derive(Deserialize)]
pub struct RequestBody {
    pub server_ip: String,
    pub map_name: String,
}

#[get("/{id}")]
async fn fetch_match(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let id = match id_str.parse::<u64>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            return HttpResponse::BadRequest().json("ID must be a valid integer");
        }
    };

    let mut existing_match = match matches::fetch_extended_match(&state, id).await {
        Ok(a_match) => a_match,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to fetch match: {}", e));
        }
    };

    let existing_match_details =
        match match_details::fetch_extended_match_details(&state, existing_match.id).await {
            Ok(details) => details,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(format!("Failed to fetch match details: {}", e));
            }
        };

    existing_match.match_details = existing_match_details;

    HttpResponse::Ok().json(existing_match)
}

#[post("/")]
async fn create_match(state: Data<AppState>, body: Json<RequestBody>) -> impl Responder {
    match matches::create(
        &state,
        body.map_name.to_string(),
        body.server_ip.to_string(),
    )
    .await
    {
        Ok(new_match) => HttpResponse::Created().json(new_match),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to create match: {}", e));
        }
    }
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/matches")
        .service(fetch_match)
        .service(create_match);

    conf.service(scope);
}
