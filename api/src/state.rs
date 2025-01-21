use std::sync::Arc;

use shared::{error::KafkaError};

use crate::{KafkaConfig, KafkaProducer};


pub struct AppState {
    pub kafka_producer: Arc<KafkaProducer>,
}

impl AppState {
    pub async fn new(config: &KafkaConfig) -> Result<Self, KafkaError> {
        let producer = KafkaProducer::new(&config)?;

        Ok(Self {
            kafka_producer: Arc::new(producer),
        })
    }
}