pub mod match_details;
pub mod matches;
pub mod players;

use std::net::Ipv4Addr;

use actix_cors::Cors;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    body::BoxBody,
    http::header::ContentType,
    middleware::Logger,
    web::{self, Data},
};

use crate::{api, lifecycle, settings::AppSettings};

pub async fn serve(settings: &AppSettings) -> anyhow::Result<()> {
    let state = lifecycle::initialize_state(settings).await?;

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(state.clone()))
            .configure(api::match_details::router)
            .configure(api::matches::router)
            .configure(api::players::router)
            .route("/", web::get().to(hello))
    })
    .bind((Ipv4Addr::UNSPECIFIED, settings.app_port))?
    .run()
    .await?;
    Ok(())
}

async fn hello(_req: HttpRequest) -> HttpResponse<BoxBody> {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello World!".to_string())
}
