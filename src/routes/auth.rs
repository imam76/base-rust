use axum::{Router, routing::get};

use crate::handlers::auth::post_auth;

pub async fn auth_route() -> Router {
    Router::new().route("/auth", get(post_auth))
}
