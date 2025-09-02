use actix_web::{
    post,
    web::{self, Data, Json},
};
use serde::Deserialize;

use crate::{
    common::{error::ServiceResponse, state::State},
    usecases,
};

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
    state: Data<State>,
    details: Json<Vec<RequestBody>>,
) -> ServiceResponse<()> {
    usecases::match_details::create_match_details(&state, Json(details.clone())).await?;
    usecases::match_details::process_match(&state, details[0].match_id).await?;

    Ok(Json(()))
}

pub fn router(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/match_details").service(create_match_details);

    conf.service(scope);
}
