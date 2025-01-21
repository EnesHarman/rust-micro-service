use rdkafka::error::KafkaError as RdKafkaError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KafkaError {
    #[error("Failed to create producer: {0}")]
    Producer(RdKafkaError),

    #[error("Failed to create consumer: {0}")]
    Consumer(RdKafkaError),

    #[error("Failed to subscribe to topics: {0}")]
    Subscription(RdKafkaError),

    #[error("Failed to produce message: {0}")]
    MessageProduction(RdKafkaError),

    #[error("Failed to consume message: {0}")]
    MessageConsumption(RdKafkaError),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}