// use crate::{AppError, Result};
use axum::{
    Json, body,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;

use crate::AppError;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn post_auth(
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Response, AppError> {
    info!("-> HANDLER - api /auth");
    let Json(body) = body.map_err(|_| AppError::BadRequest)?;

    info!("Login request: {:?}", body);
    // Here you would typically validate the credentials against a database
    // if body.email == "

    let msg = format!("Login dengan email: {}", body.email);

    let response_body = json!({
        "message": msg,
        "email": body.email,
    });
    Ok((StatusCode::OK, Json(response_body)).into_response())
}
