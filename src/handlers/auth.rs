// use crate::{AppError, Result};
use axum::{
    Json, body,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;

use crate::{
    AppError,
    models::{AppState, User},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn get_auth(
    State(state): State<AppState>,
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Response, AppError> {
    info!("-> HANDLER - api /auth");
    let Json(body) = body.map_err(|_| AppError::BadRequest)?;
    let email = body.email.trim();
    info!("Login request: {:?}", body);
    // Here you would typically validate the credentials against a database
    // if body.email == "

    //get records
    let record = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, password_hash
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        info!("Database error: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    println!("Record found: {:?}", record);

    let msg = format!("Login dengan email: {}", body.email);

    let response_body = json!({
        "message": msg,
        "email": body.email,
    });
    Ok((StatusCode::OK, Json(response_body)).into_response())
}
