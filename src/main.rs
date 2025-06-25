use anyhow::{Context, Result};
use axum::{Json, Router, extract::Extension, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{Level, info};
use tracing_subscriber;

mod errors;
mod handlers;
mod models;
mod res;
mod routes;

use crate::{models::User, routes::api_v1_routes};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load environment variables
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string());

    // Database connection
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    info!("✅ Connected to database");

    // Build application routes
    let app = Router::new()
        // Root routes
        .route("/", get(root_handler))
        .route("/health", get(simple_health))
        
        // Legacy routes (consider moving to API v1)
        .route("/hi", get(hi))
        .route("/users", get(get_all_users))
        
        // API routes
        .nest("/api/v1", api_v1_routes())
        
        // Middleware
        .layer(Extension(db_pool));

    // Start server
    let listener_address = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&listener_address).await?;
    
    info!("🚀 Server running on http://{}", listener_address);
    info!("📚 API Documentation: http://{}/api/v1/health", listener_address);
    
    axum::serve(listener, app).await?;
    Ok(())
}

// ========================
// ROOT HANDLERS
// ========================

async fn root_handler() -> &'static str {
    "🚀 Rust Base API - Ready to serve!"
}

async fn simple_health() -> &'static str {
    "OK"
}

async fn hi() -> &'static str {
    "hi!"
}

async fn get_all_users(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(User, "SELECT id, username, email, password_hash FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
