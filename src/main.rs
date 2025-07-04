use axum::{
    middleware::{self},
    response::Response,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_cookies::CookieManagerLayer;
// use serde_json::json;
// use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};

use crate::{
    handlers::health,
    middlewares::{auth_resolver_middleware, logging_middleware},
    models::AppState,
};

pub use self::errors::{AppError, Result};

mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

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

    // ✅ Create separate routers for public and protected routes
    let public_routes = Router::new()
        .route("/", get(|| async { "🚀 Welcome to the My Rust Base API!" })) // Root route
        .route("/version", get(health::get_version))
        .route("/health", get(health::get_health))
        .merge(routes::auth::routes().await); // Auth endpoints (login/logout)

    let protected_routes = Router::new()
        .nest("/api/v1/", routes::main::routes().await)
        .layer(middleware::from_fn(auth_resolver_middleware::start)); // Auth only for protected routes

    let app = Router::new()
        .merge(public_routes) // Public routes without auth
        .merge(protected_routes) // Protected routes with auth
        .layer(CookieManagerLayer::new()) // Handle cookies for all routes
        .layer(middleware::from_fn(logging_middleware::start)) // Log all requests
        .layer(middleware::map_response(main_response_mapper)) // Response mapping for all
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    info!("🚀 Server running on http://{}", &addr);
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn main_response_mapper(res: Response) -> Response {
    info!(
        "Response main_response_mapper =>: {:?}",
        "Response received"
    );
    res
}
