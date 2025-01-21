use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use event::{health_check, AppState, EventConsumer, EventService};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let appState = AppState::new().await?;
    let eventService = Arc::new(EventService::new(Arc::clone(&appState.mongo_client)));

    let event_consumer = EventConsumer::new(Arc::clone(&eventService));
    let _consumer_handle = event_consumer.start();

    HttpServer::new( move || {
        App::new()
            .service(health_check)
    }).bind("127.0.0.1:8081")?
        .run()
        .await
}
