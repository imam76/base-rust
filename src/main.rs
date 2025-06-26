use anyhow::{Context, Result};
use axum::{Router, extract::Extension, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};
use tracing_subscriber;

mod errors;
mod handlers;
mod models;
mod res;
mod routes;

use crate::routes::api_v1_routes;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load environment variables
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let port = std::env::var("PORT").unwrap_or_else(|_| "5000".to_string());

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
        // API routes
        .nest("/api/v1", api_v1_routes())
        // Middleware
        .layer(Extension(db_pool));

    // Start server
    let listener_address = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&listener_address).await?;

    info!("🚀 Server running on http://{}", listener_address);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler() -> &'static str {
    "🚀 Rust Base API - Ready to serve!"
}
