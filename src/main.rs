use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
}
