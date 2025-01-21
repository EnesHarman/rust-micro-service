use std::sync::Arc;
use mongodb::Client;
use shared::model::event::Event;

pub struct EventService {
    pub mongo_client: Arc<Client>,
}

impl EventService {
    pub fn new(mongo_client: Arc<Client>) -> Self {
        Self { mongo_client }
    }

    pub async fn save_event(&self, event: Event) -> Result<(), ()> {
        let db = self.mongo_client.database("event_handler");
        let collection = db.collection("events");
        match collection.insert_one(event).await   {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to save event: {}", err);
                Err(())
            },
        }

    }
}