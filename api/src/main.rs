use actix_web::{middleware, web, App, HttpServer};
use api::event::fire_event;
use api::health_check::health_check;
use api::{AppState, EventService, Settings};
use tracing_actix_web::TracingLogger;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let subsriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();

    let settings = Settings::new().expect("Failed to load configuration");

    info!(
        "Starting server at {}:{}",
        settings.server.host, settings.server.port
    );

    let state = AppState::new(&settings.kafka)
        .await
        .expect("Failed to initialize application state");

    

    let event_service = web::Data::new(EventService::new(Arc::clone(&state.kafka_producer)));
    HttpServer::new(move || {
        App::new()
        .wrap(TracingLogger::default())
        .wrap(middleware::Compress::default())
        .wrap(middleware::NormalizePath::trim())
        .wrap(
            middleware::DefaultHeaders::new()
                .add(("X-Version", env!("CARGO_PKG_VERSION")))
                .add(("X-Content-Type-Options", "nosniff"))
        )
            .app_data(event_service.clone())
            .service(health_check)
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/event")
                            .configure(event_config)
                    )
            )
            .default_service(
                web::route().to(|| async { 
                    actix_web::HttpResponse::NotFound().finish() 
                })
            )
    })
    .bind(format!("{}:{}", settings.server.host, settings.server.port))?
    .run()
    .await
}

fn event_config(config: &mut web::ServiceConfig) {
    config.service(fire_event);
}
