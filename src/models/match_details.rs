use serde::Serialize;

use crate::entities::match_details::MatchDetail as MatchDetailEntity;

#[derive(Serialize)]
pub struct MatchDetail {
    pub id: u64,
    pub player_id: u64,
    pub steam_name: String,
    pub steam_id: String,
    pub steam_avatar_url: String,
    pub match_id: u64,
    pub frags: i16,
    pub deaths: i16,
    pub average_ping: u16,
    pub damage_dealt: u16,
    pub damage_taken: u16,
    pub model: String,
    pub rating_after_match: f64,
    pub rating_delta: f64,
}

impl From<MatchDetailEntity> for MatchDetail {
    fn from(value: MatchDetailEntity) -> Self {
        Self {
            id: value.id,
            player_id: value.player_id,
            steam_name: value.steam_name,
            steam_id: value.steam_id,
            steam_avatar_url: value.steam_avatar_url,
            match_id: value.match_id,
            frags: value.frags,
            deaths: value.deaths,
            average_ping: value.average_ping,
            damage_dealt: value.damage_dealt,
            damage_taken: value.damage_taken,
            model: value.model,
            rating_after_match: value.rating_after_match,
            rating_delta: value.rating_delta,
        }
    }
}
