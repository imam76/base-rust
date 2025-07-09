use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::AppState,
    utils::{
        global_function::{determine_code, CodeRequest},
        ValidatedJson,
    },
    AppError,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GetCodeRequest {
    pub text: String,

    pub module: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CodeResponse {
    pub code: String,
}

pub async fn code_generator(
    state: State<AppState>,
    ValidatedJson(create_data): ValidatedJson<GetCodeRequest>,
) -> Result<Json<CodeResponse>, AppError> {
    // Extract table name from URI path
    let table_name = create_data.module.clone();

    // ✅ Much cleaner and easier to read!
    let contact_code = determine_code(
        CodeRequest {
            text: create_data.text.clone(),
            code: None, // No code provided, generate new one
        },
        &table_name,
        &state.db,
    )
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Json(CodeResponse { code: contact_code }))
}
