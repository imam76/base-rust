use axum::Json;
use serde_json::{json, Value};
use tracing::info;

pub async fn get_version() -> Json<Value> {
    info!("-> HANDLER - GET /version");

    let version_info = json!({
        "success": true,
        "app_name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "build_date": env!("CARGO_PKG_VERSION_MAJOR").to_string() + "." + env!("CARGO_PKG_VERSION_MINOR") + "." + env!("CARGO_PKG_VERSION_PATCH"),
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
        "environment": std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        "debug": std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true",
        "status": "running",
        "uptime": format!("{:?}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap())
    });

    Json(version_info)
}

pub async fn get_health() -> Json<Value> {
    info!("-> HANDLER - GET /health");

    Json(json!({
        "success": true,
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
