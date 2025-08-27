use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Data, Json},
};
use serde::Deserialize;

use crate::{common::state::AppState, repositories::match_details, usecases};

#[derive(Deserialize, Clone)]
pub struct RequestBody {
    pub steam_id: String,
    pub match_id: u64,
    pub frags: i16,
    pub deaths: i16,
    pub average_ping: u16,
    pub damage_dealt: u16,
    pub damage_taken: u16,
    pub model: String,
}

#[post("/")]
async fn create_match_details(
    state: Data<AppState>,
    details: Json<Vec<RequestBody>>,
) -> impl Responder {
    match usecases::match_details::validate_teams(&details) {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(format!("{}", e));
        }
    }

    for detail in details.iter() {
        match match_details::create(
            &state,
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
        .await
        {
            Ok(_) => continue,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(format!("Failed to create match details: {}", e));
            }
        };
    }

    if let Err(e) = usecases::match_details::process_match(&state, details[0].match_id).await {
        return HttpResponse::InternalServerError().json(format!("Failed to process match: {}", e));
    }

    HttpResponse::Created().json("Match details created successfully")
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/match_details").service(create_match_details);

    conf.service(scope);
}
