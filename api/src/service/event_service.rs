use std::sync::Arc;
use shared::error::KafkaError;
use shared::KafkaProducer;
use shared::model::event::Event;

pub struct EventService {
    kafka_producer: Arc<KafkaProducer>,
}

impl EventService {
    pub fn new(kafka_producer: Arc<KafkaProducer>) -> Self {
        Self { kafka_producer }
    }

    pub async fn handle_event(&self, event: &Event) -> Result<(), KafkaError> {
        let payload = serde_json::to_string(event).map_err(|err| {
            KafkaError::InvalidMessage("Failed to serialize event".to_string())
        })?;
        self.kafka_producer
            .send_message("event", None, payload.as_str())
            .await?;
        Ok(())
    }
}
