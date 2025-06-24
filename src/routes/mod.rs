use crate::handlers::account_subclassification;
use axum::{Router, routing::get};

pub fn api_v1_routes() -> Router {
    Router::new()
        .route("/account_classification", get(hi))
        .route(
            "/account_subclassifications",
            get(account_subclassification::asd()).post(account_subclassification::create),
        )
}

async fn hi() -> &'static str {
    "hi!"
}
