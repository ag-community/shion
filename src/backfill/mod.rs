use crate::{
    adapters::agdb_api,
    lifecycle,
    repositories::{players, stats},
    settings::AppSettings,
};

// XXX: This is temporary to backfill stats for existing players that were created before we had the stats table.
// Once all players have stats we can remove this.
pub async fn backfill_stats(settings: &AppSettings) -> anyhow::Result<()> {
    info!("Starting stats backfill...");

    let state = lifecycle::initialize_state(settings).await?;

    let players = players::fetch_all(&state).await?;

    for player in players {
        info!("Backfilling stats for player ID: {}", player.id);

        stats::update_stats(&state, player.id, 1000.0, 333.33333, 0, 0, 0, 0).await?;
    }

    info!("Stats backfill completed.");

    Ok(())
}

// XXX: This is temporary to backfill countries for existing players that were created before we had the country field in the players table.
// Once all players have a country we can remove this.
pub async fn backfill_countries(settings: &AppSettings) -> anyhow::Result<()> {
    info!("Starting countries backfill...");

    let state = lifecycle::initialize_state(settings).await?;

    let players = players::fetch_all_with_unknown_country(&state).await?;

    for player in players {
        if player.country == "xx" {
            info!(
                "Requesting info from AGDB API for player ID: {}",
                player.steam_id
            );
            match agdb_api::fetch_player_info(player.steam_id).await {
                Ok(agdb_player) => {
                    info!(
                        "Backfilling country for player ID: {} to {}",
                        agdb_player.steam_id,
                        agdb_player.country.to_lowercase()
                    );
                    players::update_country(&state, player.id, agdb_player.country.to_lowercase())
                        .await?;
                }
                Err(e) => {
                    warn!("{}", e);
                }
            };
        }
    }

    info!("Countries backfill completed.");

    Ok(())
}
