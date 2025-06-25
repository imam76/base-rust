use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::{error, warn};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub rejected_value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub validation_errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
    pub request_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum AppError {
    // Validation errors
    Validation(ValidationErrors),
    InvalidInput {
        field: String,
        message: String,
    },

    // Business logic errors
    NotFound {
        entity: String,
        id: String,
    },
    AlreadyExists {
        entity: String,
        field: String,
        value: String,
    },
    BusinessRule(String),

    // Database errors
    Database(sqlx::Error),

    // System errors
    Internal(String),
}

impl AppError {
    pub fn not_found(entity: &str, id: &str) -> Self {
        AppError::NotFound {
            entity: entity.to_string(),
            id: id.to_string(),
        }
    }

    pub fn already_exists(entity: &str, field: &str, value: &str) -> Self {
        Self::AlreadyExists {
            entity: entity.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }

    pub fn business_rule(message: &str) -> Self {
        Self::BusinessRule(message.to_string())
    }

    pub fn invalid_input(field: &str, message: &str) -> Self {
        Self::InvalidInput {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    pub fn internal(message: &str) -> Self {
        Self::Internal(message.to_string())
    }

    // Get error code for consistent identification
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::InvalidInput { .. } => "INVALID_INPUT",
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::AlreadyExists { .. } => "ALREADY_EXISTS",
            AppError::BusinessRule(_) => "BUSINESS_RULE_VIOLATION",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    // Get HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidInput { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::AlreadyExists { .. } => StatusCode::CONFLICT,
            AppError::BusinessRule(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // Get user-friendly message
    pub fn user_message(&self) -> String {
        match self {
            AppError::Validation(_) => "Validation failed. Please check your input.".to_string(),
            AppError::InvalidInput { field, message } => format!("Invalid {}: {}", field, message),
            AppError::NotFound { entity, id } => format!("{} with ID '{}' not found", entity, id),
            AppError::AlreadyExists {
                entity,
                field,
                value,
            } => {
                format!("{} with {} '{}' already exists", entity, field, value)
            }
            AppError::BusinessRule(message) => message.clone(),
            AppError::Database(_) => {
                "A database error occurred. Please try again later.".to_string()
            }
            AppError::Internal(_) => {
                "An internal error occurred. Please try again later.".to_string()
            }
        }
    }

    // Convert validation errors to our format
    fn validation_errors_to_vec(errors: &ValidationErrors) -> Vec<ValidationError> {
        errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| ValidationError {
                    field: field.to_string(),
                    message: error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid value for field '{}'", field)),
                    rejected_value: error.params.get("value").cloned(),
                })
            })
            .collect()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4(); // In real app, get from middleware

        // Log error based on severity
        match &self {
            AppError::Database(e) => {
                error!(
                    request_id = %request_id,
                    error = %e,
                    "Database error occurred"
                );
            }
            AppError::Internal(message) => {
                error!(
                    request_id = %request_id,
                    message = %message,
                    "Internal error occurred"
                );
            }
            AppError::Validation(errors) => {
                warn!(
                    request_id = %request_id,
                    errors = ?errors,
                    "Validation error"
                );
            }
            _ => {
                warn!(request_id = %request_id, error = ?self, "Application error");
            }
        }

        let error_detail = ErrorDetail {
            code: self.code().to_string(),
            message: self.user_message(),
            details: None,
            validation_errors: match &self {
                AppError::Validation(errors) => Some(Self::validation_errors_to_vec(errors)),
                _ => None,
            },
        };

        let error_response = ErrorResponse {
            success: false,
            error: error_detail,
            request_id,
            timestamp: chrono::Utc::now(),
        };

        (self.status_code(), Json(error_response)).into_response()
    }
}

// From implementations for easy conversion
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        Self::Validation(error)
    }
}

// Type alias for convenience
pub type AppResult<T> = Result<T, AppError>;
