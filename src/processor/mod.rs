use anyhow::Ok;

use crate::{
    lifecycle,
    repositories::{matches, players},
    settings::AppSettings,
    usecases::match_details,
};

pub async fn reprocess_all(settings: &AppSettings) -> anyhow::Result<()> {
    tracing::info!("Starting historical data processing...");

    let state = lifecycle::initialize_state(&settings).await?;

    let matches = matches::fetch_all_matches(&state).await?;

    let _ = players::reset_all_player_ratings(&state).await?;

    for match_entry in matches {
        tracing::info!("Reprocessing match ID: {}", match_entry.id);

        let _ = match_details::process_match(&state, match_entry.id).await;
    }

    tracing::info!("Historical data processing completed.");

    Ok(())
}
