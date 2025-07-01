use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
// use serde_json::json;
// use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};

use crate::models::AppState;

pub use self::errors::{AppError, Result};

mod errors;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "5001".to_string());
    let addr = format!("127.0.0.1:{}", port);

    // Database connection
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");
    info!("✅ Connected to database{}", db_url);

    // Create app state
    let app_state = AppState { db: db_pool };

    let app = Router::new()
        .nest("/api/v1/auth", routes::auth::routes().await)
        .nest("/api/v1/", routes::main::routes().await)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    info!("🚀 Server running on http://{}", &addr);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
