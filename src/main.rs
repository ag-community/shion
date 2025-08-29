use core::panic;
use shion::settings::AppSettings;
use shion::{api, backfill, lifecycle, processor};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let settings = AppSettings::get();
    lifecycle::initialize_logging(&settings);

    match settings.app_component.as_str() {
        "api" => api::serve(settings).await,
        "backfill" => backfill::backfill_stats(&settings).await,
        "processor" => processor::reprocess_all(settings).await,
        _ => panic!("Unknown app component"),
    }
}
