use crate::{
    lifecycle,
    repositories::{players, stats},
    settings::AppSettings,
};

// XXX: This is temporary to backfill stats for existing players that were created before we had the stats table.
// Once all players have stats we can remove this.
pub async fn backfill_stats(settings: &AppSettings) -> anyhow::Result<()> {
    info!("Starting stats backfill...");

    let state = lifecycle::initialize_state(&settings).await?;

    let players = players::fetch_all(&state).await?;

    for player in players {
        info!("Backfilling stats for player ID: {}", player.id);

        let _ = stats::update_stats(&state, player.id, 1000.0, 333.33333, 0, 0, 0, 0).await?;
    }

    info!("Stats backfill completed.");

    Ok(())
}
