use serde::Deserialize;
use config::{Config, ConfigError, Environment};


#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_server: String, 
    pub group_id: String,
    pub client_id: String,
}


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig, 
    pub kafka: KafkaConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()

    }
}