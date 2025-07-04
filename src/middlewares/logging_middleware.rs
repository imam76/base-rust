use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tracing::info;

// Middleware function
pub async fn start(request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    println!("Request: {} {}", method, uri);
    info!("LOGGING_MIDDLEWARE - {} {}", method, uri);

    let response = next.run(request).await;

    response
}
