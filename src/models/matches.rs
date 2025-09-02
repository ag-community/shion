use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    entities::match_details::MatchDetail as MatchDetailEntity,
    entities::matches::Match as MatchEntity, models::match_details::MatchDetail,
};

#[derive(Serialize)]
pub struct Match {
    pub id: u64,
    pub server_ip: String,
    pub map_name: String,
}

#[derive(Serialize)]
pub struct MatchExtended {
    pub id: u64,
    pub server_ip: String,
    pub match_date: DateTime<Utc>,
    pub map_name: String,
    pub match_details: Vec<MatchDetail>,
}

impl From<MatchEntity> for Match {
    fn from(value: MatchEntity) -> Self {
        Self {
            id: value.id,
            server_ip: value.server_ip,
            map_name: value.map_name,
        }
    }
}

impl From<(MatchEntity, Vec<MatchDetailEntity>)> for MatchExtended {
    fn from(value: (MatchEntity, Vec<MatchDetailEntity>)) -> Self {
        let (match_value, match_details_value) = value;
        Self {
            id: match_value.id,
            server_ip: match_value.server_ip,
            match_date: match_value.match_date,
            map_name: match_value.map_name,
            match_details: match_details_value
                .into_iter()
                .map(MatchDetail::from)
                .collect(),
        }
    }
}
