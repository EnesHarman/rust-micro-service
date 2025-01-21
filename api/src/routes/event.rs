use crate::EventService;
use actix_web::{post, web, HttpResponse, Result};
use serde_json::json;
use shared::model::event::Event;
use tracing::{error, info, instrument};

#[post("/fire")]
#[instrument(skip(event_service, event_dto))]
pub async fn fire_event(
    event_service: web::Data<EventService>,
    event_dto: web::Json<EventDto>,
) -> Result<HttpResponse> {
    info!("Received event request");
    let event = event_dto.into_inner().into_event();
    match event_service.handle_event(&event).await {
        Ok(_) => {
            info!("Successfully processed event");
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Event processed successfully"
            })))
        }
        Err(e) => {
            error!("Failed to process event: {}", e);
            Err(actix_web::error::ErrorInternalServerError(json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct EventDto {
    pub code: String,
    pub user_id: i32,
    pub ts: i64,
    pub timezone: String,
}

impl EventDto {
    pub fn into_event(self) -> Event {
        Event {
            _id: None,
            user_id: self.user_id,
            code: self.code,
            ts: self.ts,
            timezone: self.timezone,
        }
    }
}
