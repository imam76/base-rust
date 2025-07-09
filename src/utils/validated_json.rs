use axum::{
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;

use crate::AppError;

pub struct ValidatedJson<T>(pub T);

fn clean_error_message(error_msg: &str) -> String {
    // Extract just the essential part of the error message
    if let Some(missing_start) = error_msg.find("missing field") {
        if let Some(missing_end) = error_msg[missing_start..].find(" at line") {
            return format!(
                "Invalid JSON: {}",
                &error_msg[missing_start..missing_start + missing_end]
            );
        }
        // If no " at line" found, take everything after "missing field"
        if let Some(quote_end) = error_msg[missing_start..].find('`') {
            if let Some(second_quote) = error_msg[missing_start + quote_end + 1..].find('`') {
                let field_name = &error_msg
                    [missing_start + quote_end + 1..missing_start + quote_end + 1 + second_quote];
                return format!("Invalid JSON: missing field `{}`", field_name);
            }
        }
    }

    // For other errors, just return a generic message
    "Invalid JSON: Failed to parse request body".to_string()
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::BadRequest(clean_error_message(&err.to_string())))?;

        Ok(ValidatedJson(value))
    }
}
