use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

use super::config::KafkaConfig;
use crate::error::KafkaError;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(config: &KafkaConfig) -> Result<Self, KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("client.id", &config.client_id)
            .create()
            .map_err(KafkaError::Producer)?;

        Ok(Self { producer })
    }

    pub async fn send_message(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &str,
    ) -> Result<(), KafkaError> {
        let mut record = FutureRecord::to(topic).payload(payload);
        if let Some(k) = key {
            record = record.key(k);
        }

        self.producer
            .send(record, Duration::from_secs(5))
            .await
            .map_err(|(err, _)| KafkaError::MessageProduction(err))?;

        Ok(())
    }
}