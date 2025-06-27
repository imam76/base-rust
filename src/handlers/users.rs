use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {
    name: String,
    message: String,
}

pub async fn get_all_users(Query(params): Query<Hello>) -> impl IntoResponse {
    println!("->>{:<12} - handler called", "GET /");

    let name = params.name;
    let message = params.message;
    Html(format!("<h1>Hello, {}!</h1><p>{}</p>", name, message))
}
