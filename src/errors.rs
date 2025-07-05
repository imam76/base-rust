use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::info;

#[derive(Debug)]
pub enum AppError {
    UnhandledError(String),
    LoginFailed,
    BadRequest(String),
    DatabaseError(String),
    UnAuthorized,
    CookieFormatError,
    NotFound { id: uuid::Uuid },
    SerializationError(String),
    InternalServerError(String),
}

pub type Result<T> = core::result::Result<T, AppError>;

// Macro to generate error responses
macro_rules! error_response {
    ($status:expr, $error:expr, $details:expr) => {{
        info!("Error: {:<12} - {}", "INTO RES", $error);
        let body = json!({
            "error": $error,
            "details": $details
        });
        ($status, Json(body)).into_response()
    }};
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(details) => {
                error_response!(StatusCode::BAD_REQUEST, "Bad Request", details)
            }
            AppError::LoginFailed => error_response!(
                StatusCode::UNAUTHORIZED,
                "Login Failed",
                "Invalid email or password."
            ),
            AppError::UnAuthorized => error_response!(
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "You must be authenticated to access this resource."
            ),
            AppError::CookieFormatError => error_response!(
                StatusCode::BAD_REQUEST,
                "Cookie Format Error",
                "The provided cookie format is invalid."
            ),
            AppError::DatabaseError(details) => {
                error_response!(StatusCode::INTERNAL_SERVER_ERROR, "Database Error", details)
            }
            AppError::UnhandledError(details) => {
                error_response!(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unhandled Error",
                    details
                )
            }
            AppError::NotFound { id } => error_response!(
                StatusCode::NOT_FOUND,
                "Not Found",
                format!("Resource with id {} not found", id)
            ),
            AppError::SerializationError(details) => error_response!(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Serialization Error",
                details
            ),
            AppError::InternalServerError(details) => error_response!(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                details
            ),
        }
    }
}
