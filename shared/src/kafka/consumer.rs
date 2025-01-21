use futures::StreamExt;
use rdkafka::{
    consumer::{StreamConsumer, Consumer},
    ClientConfig, Message
};
use rdkafka::consumer::DefaultConsumerContext;
use rdkafka::util::DefaultRuntime;
use crate::error::KafkaError;

use super::KafkaConfig;

pub struct KafkaConsumer {
    consumer: StreamConsumer
}

impl KafkaConsumer {
    pub fn build(config: KafkaConfig, topic: String) -> Result<Self, KafkaError> {
        let consumer: StreamConsumer<DefaultConsumerContext, DefaultRuntime> = ClientConfig::new()
            .set("bootstrap.servers", config.bootstrap_servers)
            .set("group.id", config.group_id)
            .set("client.id", config.client_id)
            .create()
            .map_err(KafkaError::Consumer)?;

        consumer.subscribe(&[topic.as_str()]).map_err(KafkaError::Subscription)?;
        Ok(Self { consumer })
    }

    pub async fn consume_messages<F, Fut>(&self, mut callback: F) -> Result<(), KafkaError> 
    where
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = Result<(), KafkaError>>,
    {
        let mut message_stream = self.consumer.stream();

        while let Some(message_result) = message_stream.next().await {
            let message = message_result.map_err(KafkaError::MessageConsumption)?;
            
            let payload = message
                .payload()
                .and_then(|p| String::from_utf8(p.to_vec()).ok())
                .ok_or(KafkaError::InvalidMessage("No payload".to_string()))?;

            callback(payload).await?;
        }

        Ok(())
    }
}