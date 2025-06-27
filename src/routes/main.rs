use axum::{Router, response::IntoResponse, routing::get};

use crate::handlers::{root::get_all, users::get_all_users};

pub async fn route() -> Router {
    Router::new()
        .route("/", get(get_all))
        .route("/hello", get(get_all_users))
}
