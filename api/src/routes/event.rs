
use actix_web::{post, web, HttpResponse, Result};
use shared::model::event::Event;
use crate::{ EventService};

#[post("/fire")]
pub async fn fire_event(event_service: web::Data<EventService>, event_dto: web::Json<EventDto>) -> Result<HttpResponse, > {
    let event = event_dto.into_inner().into_event();
    event_service
        .handle_event(&event)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().finish())

}

#[derive(serde::Deserialize)]
pub struct EventDto { 
    pub code: String,
    pub user_id: i32,
    pub ts: i64,
    pub timezone: String
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