use std::sync::Arc;
use mongodb::Client;

pub struct AppState {
    pub mongo_client: Arc<Client>,
}

impl AppState {
    pub async fn new() -> Result<Self, std::io::Error> {
        let mongo_uri = "mongodb://admin:password@localhost:27017/";
        let client = Client::with_uri_str(mongo_uri).await.map_err(|err| {
            eprintln!("Failed to connect to MongoDB: {}", err);
            Err(err)
        }).unwrap_or_else(|err: Result<(), mongodb::error::Error>| {
           panic!("Failed to connect to MongoDB: {:?}", err);
        });

        Ok(Self {
            mongo_client: Arc::new(client),
        })
    }
}