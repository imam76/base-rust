use axum::response::{Html, IntoResponse};

pub async fn get_all() -> impl IntoResponse {
    println!("->>{:<12} - root called", "GET /");
    Html("Let's get RUST 🚀".to_string())
}
