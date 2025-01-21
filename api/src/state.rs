use std::sync::Arc;

use shared::{error::KafkaError, kafka::{KafkaConfig, KafkaProducer}};

pub struct AppState {
    pub kafka_producer: Arc<KafkaProducer>,
}

impl AppState {
    pub async fn new() -> Result<Self, KafkaError> {
        let config = KafkaConfig::new(
            "localhost:9092".to_string(),
            "event-group".to_string(), 
            "event-client".to_string()
        );
        let producer = KafkaProducer::new(&config)?;

        Ok(Self {
            kafka_producer: Arc::new(producer),
        })
    }
}