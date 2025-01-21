use std::sync::Arc;
use shared::error::KafkaError;
use shared::model::event::Event;
use tracing::{error, info, instrument};

use crate::KafkaProducer;

pub struct EventService {
    kafka_producer: Arc<KafkaProducer>,
}

impl EventService {
    pub fn new(kafka_producer: Arc<KafkaProducer>) -> Self {
        Self { kafka_producer }
    }

    #[instrument(skip(self, event), fields(user_id = event.user_id))]
    pub async fn handle_event(&self, event: &Event) -> Result<(), KafkaError> {
        info!("Processing event with code: {}", event.code);
        let payload = serde_json::to_string(event).map_err(|err| {
            error!("Failed to serialize event: {}", err);
            KafkaError::InvalidMessage(err.to_string())
        })?;
        self.kafka_producer
            .send_message("event", None, payload.as_str())
            .await.map_err(|e| {
                error!("Failed to send message to Kafka: {}", e);
                e
            })?;
            info!("Successfully processed event");
        Ok(())
    }
}
