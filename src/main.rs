use core::panic;
use shion::settings::AppSettings;
use shion::{api, processor};

// https://github.com/wpcodevo/simple-api-actix-web/blob/master/src/main.rs
// https://github.com/Srinivasa314/actix-web-example/blob/36333e8a1b50ddad6e6c1c541f8e510773024ab4/src/main.rs
// https://github.com/daniel-samson/http-api-rs/tree/main
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let settings = AppSettings::get();
    match settings.app_component.as_str() {
        "api" => api::serve(settings).await,
        "processor" => processor::reprocess_all(settings).await,
        _ => panic!("Unknown app component"),
    }
}
