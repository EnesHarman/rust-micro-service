pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub client_id: String,
}

impl KafkaConfig {
    pub fn new(bootstrap_servers: String, group_id: String, client_id: String) -> Self {
        Self { bootstrap_servers, group_id, client_id }
    }
}