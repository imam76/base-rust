use anyhow::{Context, Result};
use axum::{Extension, Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
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
