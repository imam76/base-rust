// use crate::{AppError, Result};
extern crate bcrypt;
use bcrypt::verify;

use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{
    models::{AppState, User},
    utils::{self, constants::AUTH_TOKEN},
    AppError,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn get_auth(
    cookies: Cookies,
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
        SELECT id, username, email, password_hash
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

    let valid = verify(&body.password, &record.password_hash)
        .map_err(|e| AppError::UnhandledError(e.to_string()))?;

    if valid {
        info!("Password is valid for user: {}", body.email);
        let auth_tokken = format!("user-{}.exp.sign", record.id);
        cookies.add(Cookie::new(utils::constants::AUTH_TOKEN, auth_tokken));
    } else {
        info!("Invalid password for user: {}", body.email);
        cookies.remove(Cookie::from(AUTH_TOKEN));
        return Err(AppError::LoginFailed);
    }
    //fix me implementing a cookie

    if valid {
        println!("Record found: {:?}", record);
        let msg = format!("Login dengan email: {}", body.email);
        let response_body = json!({
            "message": msg,
            "email": body.email,
        });
        Ok((StatusCode::OK, Json(response_body)).into_response())
    } else {
        info!("INI GAK VALID");
        Err(AppError::LoginFailed)
    }
}
