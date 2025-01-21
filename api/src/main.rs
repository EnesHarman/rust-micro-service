use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use api::{AppState, EventService};
use api::event::fire_event;
use api::health_check::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new()
        .await
        .expect("Failed to initialize application state");

    let event_service = web::Data::new(
        EventService::new(Arc::clone(&state.kafka_producer))
    );
    HttpServer::new(move || {
        App::new()
            .app_data(event_service.clone())
            .service(health_check)
            .service(web::scope("/event")
                .configure(event_config)
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn event_config(config: &mut web::ServiceConfig) {
    config.service(fire_event);
}
