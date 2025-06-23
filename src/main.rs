use anyhow::{Context, Result};
use axum::{Json, Router, extract::Extension, http::StatusCode, routing::get};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{Level, info};
use tracing_subscriber;

mod handlers;
mod models;
mod routes;
use crate::{models::User, routes::api_v1_routes};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let db_pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/hi", get(hi))
        .route("/users", get(get_all_users))
        .nest("/api/v1", api_v1_routes())
        .layer(Extension(db_pool));

    // run our app with hyper, listening globally on port
    let listener_host = format!("127.0.0.1:{}", &port);
    let listener = tokio::net::TcpListener::bind(&listener_host).await?;
    info!("Server is running on http://127.0.0.1:{}", &port);
    axum::serve(listener, app).await?;

    Ok(())
}

// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
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
