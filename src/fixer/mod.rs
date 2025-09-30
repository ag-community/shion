use crate::{
    entities::match_details::MatchDetail,
    lifecycle,
    repositories::{match_details, matches},
    settings::AppSettings,
};

// XXX: This is temporary to fix existing matches that have uneven teams or very low frags.
// This happened due to the old api not having proper validation for these cases
pub async fn fix_matches(settings: &AppSettings) -> anyhow::Result<()> {
    info!("Starting match fixes...");

    let state = lifecycle::initialize_state(settings).await?;

    let matches = matches::fetch_all_matches(&state).await?;

    for match_entry in matches {
        let mut to_delete = false;
        let match_details = match_details::fetch_match_details(&state, match_entry.id).await?;

        if !match_details.is_empty() {
            let blue_team: Vec<MatchDetail> = match_details
                .iter()
                .filter(|detail| detail.model == "blue")
                .cloned()
                .collect();
            let red_team: Vec<MatchDetail> = match_details
                .iter()
                .filter(|detail| detail.model == "red")
                .cloned()
                .collect();

            if blue_team.len() != red_team.len() {
                warn!(
                    "Match ID: {} has uneven teams. Blue team size: {}, Red team size: {}",
                    match_entry.id,
                    blue_team.len(),
                    red_team.len()
                );
                to_delete = true;
            }

            let blue_team_frags: i16 = blue_team.iter().map(|detail| detail.frags).sum();
            let red_team_frags: i16 = red_team.iter().map(|detail| detail.frags).sum();

            if blue_team_frags < 10 && red_team_frags < 10 {
                warn!(
                    "Match ID: {} has low frags. Blue team frags: {}, Red team frags: {}",
                    match_entry.id, blue_team_frags, red_team_frags
                );
                to_delete = true;
            }

            if to_delete {
                info!("Deleting match ID: {} and its details", match_entry.id);
                match_details::delete_by_match_id(&state, match_entry.id).await?;
                matches::delete_match(&state, match_entry.id).await?;
            }
        }
    }

    info!("Match fixes completed.");

    Ok(())
}
