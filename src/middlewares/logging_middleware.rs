use axum::{body::Body, extract::Request, middleware::Next, response::Response};

// Middleware function
pub async fn start(request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    println!("Request: {} {}", method, uri);

    let response = next.run(request).await;

    println!("Response status: {}", response.status());

    response
}
