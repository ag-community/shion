use core::panic;
use shion::settings::AppSettings;
use shion::{api, backfill, fixer, lifecycle, processor};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let settings = AppSettings::get();
    lifecycle::initialize_logging(&settings);

    match settings.app_component.as_str() {
        "api" => api::serve(settings).await,
        "backfill_stats" => backfill::backfill_stats(&settings).await,
        "backfill_countries" => backfill::backfill_countries(&settings).await,
        "fix_matches" => fixer::fix_matches(&settings).await,
        "processor" => processor::reprocess_all(settings).await,
        _ => panic!("Unknown app component"),
    }
}
