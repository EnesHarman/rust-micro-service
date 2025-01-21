use std::sync::Arc;
use tokio::task::JoinHandle;
use shared::{KafkaConfig, KafkaConsumer};
use shared::model::event::Event;
use crate::EventService;
use shared::error::KafkaError;

pub struct EventConsumer {
    event_service: Arc<EventService>,
}

impl EventConsumer {
    pub fn new (event_service: Arc<EventService>) -> Self {
        Self {
            event_service,
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let config = KafkaConfig::new(
                "localhost:9092".to_string(),
                "event_consumer_group".to_string(),
                "event_consumer".to_string(),
            );

            let consumer = match KafkaConsumer::build(config, "event".to_string()) {
                Ok(consumer) => consumer,
                Err(e) => {
                    eprintln!("Failed to build Kafka consumer: {:?}", e);
                    return;
                }
            };

            let event_service = Arc::clone(&self.event_service);

            if let Err(e) = consumer.consume_messages(move |payload| {
                let event_service = Arc::clone(&event_service);
                
                async move {
                    match serde_json::from_str::<Event>(&payload) {
                        Ok(event) => {
                            println!("Event consumer received: {:?}", event);
                            match event_service.save_event(event).await {
                                Ok(_) => Ok(()),
                                Err(e) => {
                                    eprintln!("Failed to save event to MongoDB: {:?}", e);
                                    Err(KafkaError::InvalidMessage("Failed to save event to MongoDB".to_string()))
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize event: {:?}", e);
                            Ok(())
                        }
                    }
                }
            }).await {
                eprintln!("Failed to consume messages: {:?}", e);
            }
        })
    }
}