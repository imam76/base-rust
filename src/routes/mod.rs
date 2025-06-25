use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub mod account_subclassification;

/// Health check handler
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "rust-base-api",
        "version": "1.0.0"
    }))
}

/// Main API v1 router
///
/// This function combines all API v1 routes into a single router.
/// Add new route modules here to maintain a clean and organized structure.
pub fn api_v1_routes() -> Router {
    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Account subclassification routes
        .nest(
            "/account-subclassifications",
            account_subclassification::routes(),
        )
    // Add more route modules here
    // .nest("/users", user::routes())
    // .nest("/contacts", contact::routes())
}
