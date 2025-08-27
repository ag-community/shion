use skillratings::{
    Outcomes,
    weng_lin::{WengLinConfig, WengLinRating, weng_lin_two_teams},
};

#[derive(Debug)]
struct Stats {
    kills: i16,
    deaths: i16,
    damage: i16,
}

fn main() -> () {
    let blue_team = vec![WengLinRating::new(), WengLinRating::new()];
    let red_team = vec![WengLinRating::new(), WengLinRating::new()];

    let stats_blue_team = vec![
        Stats {
            kills: 30,
            deaths: 64,
            damage: 1407,
        },
        Stats {
            kills: 22,
            deaths: 48,
            damage: 1975,
        },
    ];

    let stats_red_team = vec![
        Stats {
            kills: 52,
            deaths: 41,
            damage: 1500,
        },
        Stats {
            kills: 51,
            deaths: 20,
            damage: 2140,
        },
    ];

    println!(
        "Initial Blue Team Ratings: {:?} - Computed Ratings: {:?}",
        blue_team,
        blue_team
            .iter()
            .map(|r| compute_scaled_rating(r.rating, r.uncertainty))
            .collect::<Vec<i32>>()
    );
    println!(
        "Initial Red Team Ratings: {:?} - Computed Ratings: {:?}",
        red_team,
        red_team
            .iter()
            .map(|r| compute_scaled_rating(r.rating, r.uncertainty))
            .collect::<Vec<i32>>()
    );

    let outcome = determine_winner(&stats_blue_team, &stats_red_team);

    let config = WengLinConfig::new();

    // The trueskill_two_teams function will calculate the new ratings for both teams and return them.
    let (mut new_blue_team, mut new_red_team) =
        weng_lin_two_teams(&blue_team, &red_team, &outcome, &config);

    println!(
        "New Blue Team Ratings: {:?} - Computed Ratings: {:?}",
        new_blue_team,
        new_blue_team
            .iter()
            .map(|r| compute_scaled_rating(r.rating, r.uncertainty))
            .collect::<Vec<i32>>()
    );
    println!(
        "New Red Team Ratings: {:?} - Computed Ratings: {:?}",
        new_red_team,
        new_red_team
            .iter()
            .map(|r| compute_scaled_rating(r.rating, r.uncertainty))
            .collect::<Vec<i32>>()
    );

    for (rating, stats) in new_blue_team.iter_mut().zip(stats_blue_team.iter()) {
        adjust_for_performance(rating, stats);
    }
    for (rating, stats) in new_red_team.iter_mut().zip(stats_red_team.iter()) {
        adjust_for_performance(rating, stats);
    }

    println!("\nRatings after performance adjustment:");
    for (i, r) in new_blue_team.iter().enumerate() {
        println!(
            "Blue P{} => Scaled: {} | mu {:.2} ± {:.2}",
            i + 1,
            compute_scaled_rating(r.rating, r.uncertainty),
            r.rating,
            r.uncertainty
        );
    }
    for (i, r) in new_red_team.iter().enumerate() {
        println!(
            "Red P{}  => Scaled: {} | mu {:.2} ± {:.2}",
            i + 1,
            compute_scaled_rating(r.rating, r.uncertainty),
            r.rating,
            r.uncertainty
        );
    }
}

fn determine_winner(blue_stats: &Vec<Stats>, red_stats: &Vec<Stats>) -> Outcomes {
    let blue_score: i32 = blue_stats.iter().map(|s| s.kills as i32).sum();
    let red_score: i32 = red_stats.iter().map(|s| s.kills as i32).sum();

    if blue_score > red_score {
        Outcomes::WIN
    } else {
        Outcomes::LOSS
    }
}

fn compute_scaled_rating(mu: f64, sigma: f64) -> i32 {
    let base_conservative = 25.0 - 3.0 * 25.0 / 3.0;
    let conservative = mu - 3.0 * sigma;
    (1000.0 + 10.3 * (conservative - base_conservative)).round() as i32
}

fn adjust_for_performance(rating: &mut WengLinRating, stats: &Stats) {
    let kd_ratio = stats.kills as f64 / (stats.deaths as f64 + 1.0);
    let mut kd_factor = 1.0 + (kd_ratio - 1.0) * 0.05;

    if kd_factor > 1.1 {
        kd_factor = 1.1;
    } else if kd_factor < 0.9 {
        kd_factor = 0.9;
    }

    let mut dmg_factor = 1.0 + (stats.damage as f64 / 1000.0 - 1.0) * 0.02;

    if dmg_factor > 1.1 {
        dmg_factor = 1.1;
    } else if dmg_factor < 0.9 {
        dmg_factor = 0.9;
    }

    let total_factor = kd_factor * dmg_factor;

    rating.rating *= total_factor;
}
