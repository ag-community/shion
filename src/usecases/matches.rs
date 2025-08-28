use crate::{
    common::{
        error::{AppError, ServiceResult, unexpected},
        state::DatabaseState,
    },
    entities::matches::{Match, MatchExtended},
    repositories::{match_details, matches},
};

pub async fn create_match<T: DatabaseState>(
    state: &T,
    map_name: String,
    server_ip: String,
) -> ServiceResult<Match> {
    let new_match = matches::create(state, map_name.to_string(), server_ip.to_string()).await?;
    Ok(new_match)
}

pub async fn fetch_match<T: DatabaseState>(state: &T, id: u64) -> ServiceResult<MatchExtended> {
    let mut existing_match = match matches::fetch_extended_match(state, id).await {
        Ok(a_match) => a_match,
        Err(sqlx::Error::RowNotFound) => return Err(AppError::MatchNotFound),
        Err(e) => return unexpected(e),
    };

    let existing_match_details =
        match match_details::fetch_extended_match_details(state, existing_match.id).await {
            Ok(details) => details,
            Err(sqlx::Error::RowNotFound) => return Err(AppError::MatchDetailNotFound),
            Err(e) => return unexpected(e),
        };

    existing_match.match_details = existing_match_details;

    Ok(existing_match)
}
