// src/handlers/account_subclassification.rs
use axum::{Extension, Json, extract::Path};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::{info, instrument};
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::AppError,
    models::ResAccountSubclassification,
    res::{ApiResponse, ApiResult},
};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAccountSubclassification {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Code must be between 1 and 20 characters"
    ))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(max = 255, message = "Alias name cannot exceed 255 characters"))]
    pub alias_name: Option<String>,

    #[validate(custom(function = "validate_cash_flow_type"))]
    pub cash_flow_type: String,

    #[validate(length(max = 50, message = "Ratio type cannot exceed 50 characters"))]
    pub ratio_type: Option<String>,

    pub is_variable_cost: bool,
    pub account_classification_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub is_parent: bool,
    pub is_active: bool,

    // These will be set by the system, not from request
    #[serde(skip_deserializing)]
    pub created_by: Option<Uuid>,
    #[serde(skip_deserializing)]
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateAccountSubclassification {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(max = 255, message = "Alias name cannot exceed 255 characters"))]
    pub alias_name: Option<String>,

    #[validate(custom(function = "validate_cash_flow_type"))]
    pub cash_flow_type: Option<String>,

    #[validate(length(max = 50, message = "Ratio type cannot exceed 50 characters"))]
    pub ratio_type: Option<String>,

    pub is_variable_cost: Option<bool>,
    pub parent_id: Option<Uuid>,
    pub is_parent: Option<bool>,
    pub is_active: Option<bool>,
}

// Custom validation function
fn validate_cash_flow_type(cash_flow_type: &str) -> Result<(), validator::ValidationError> {
    let valid_types = ["operating", "investing", "financing"];
    if valid_types.contains(&cash_flow_type) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_cash_flow_type"))
    }
}

/// GET All
#[instrument(skip(pool))]
pub async fn get_all(
    Extension(pool): Extension<Pool<Postgres>>,
) -> ApiResult<Vec<ResAccountSubclassification>> {
    info!("Fetching all account subclassifications");

    let records = sqlx::query_as!(
        ResAccountSubclassification,
        r#"
        SELECT id, code, name, alias_name, cash_flow_type, ratio_type, 
               is_variable_cost, account_classification_id, parent_id, 
               is_parent, is_active, created_at, created_by, updated_at, updated_by
        FROM account_subclassifications 
        WHERE is_active = true 
        ORDER BY code ASC
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(ApiResponse::success(records))
}

/// GET by id
#[instrument(skip(pool))]
pub async fn get_by_id(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> ApiResult<ResAccountSubclassification> {
    info!("Fetching account subclassification by id: {}", id);

    let record = sqlx::query_as!(
        ResAccountSubclassification,
        r#"
        SELECT id, code, name, alias_name, cash_flow_type, ratio_type, 
               is_variable_cost, account_classification_id, parent_id, 
               is_parent, is_active, created_at, created_by, updated_at, updated_by
        FROM account_subclassifications 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::not_found("Account Subclassification", &id.to_string()))?;

    Ok(ApiResponse::success(record))
}

/// POST
#[instrument(skip(pool, request))]
pub async fn create(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(mut request): Json<CreateAccountSubclassification>,
) -> ApiResult<ResAccountSubclassification> {
    info!("Creating account subclassification: {}", request.code);

    // Validate request
    request.validate()?;

    // Check for duplicate code
    let existing_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM account_subclassifications WHERE code = $1 AND is_active = true",
        request.code
    )
    .fetch_one(&pool)
    .await?;

    if existing_count.unwrap_or(0) > 0 {
        return Err(AppError::already_exists(
            "Account Subclassification",
            "code",
            &request.code,
        ));
    }

    // Business rule validation
    if request.is_parent && request.parent_id.is_some() {
        return Err(AppError::business_rule(
            "A parent account subclassification cannot have a parent",
        ));
    }

    // Set system fields (in real app, get from auth middleware)
    let current_user_id = Uuid::new_v4();
    request.created_by = Some(current_user_id);
    request.updated_by = Some(current_user_id);

    let record = sqlx::query_as!(
        ResAccountSubclassification,
        r#"
        INSERT INTO account_subclassifications (
            code, name, alias_name, cash_flow_type, ratio_type, is_variable_cost, 
            account_classification_id, parent_id, is_parent, is_active, 
            created_by, updated_by
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
        ) 
        RETURNING 
            id, code, name, alias_name, cash_flow_type, ratio_type, is_variable_cost, 
            account_classification_id, parent_id, is_parent, is_active, 
            created_at, created_by, updated_at, updated_by
        "#,
        request.code,
        request.name,
        request.alias_name,
        request.cash_flow_type,
        request.ratio_type,
        request.is_variable_cost,
        request.account_classification_id,
        request.parent_id,
        request.is_parent,
        request.is_active,
        request.created_by.unwrap(),
        request.updated_by.unwrap(),
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::already_exists("Account Subclassification", "code", &request.code)
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::business_rule("Invalid account_classification_id or parent_id reference")
        }
        _ => AppError::Database(e),
    })?;

    Ok(ApiResponse::success_with_message(
        record,
        "Account subclassification created successfully".to_string(),
    ))
}

/// PUT
#[instrument(skip(pool, request))]
pub async fn update(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateAccountSubclassification>,
) -> ApiResult<ResAccountSubclassification> {
    info!("Updating account subclassification: {}", id);

    // Validate request
    request.validate()?;

    let current_user_id = Uuid::new_v4();

    let record = sqlx::query_as!(
        ResAccountSubclassification,
        r#"
        UPDATE account_subclassifications 
        SET 
            name = COALESCE($1, name),
            alias_name = COALESCE($2, alias_name),
            cash_flow_type = COALESCE($3, cash_flow_type),
            ratio_type = COALESCE($4, ratio_type),
            is_variable_cost = COALESCE($5, is_variable_cost),
            parent_id = COALESCE($6, parent_id),
            is_parent = COALESCE($7, is_parent),
            is_active = COALESCE($8, is_active),
            updated_by = $9,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $10 AND is_active = true
        RETURNING 
            id, code, name, alias_name, cash_flow_type, ratio_type, is_variable_cost, 
            account_classification_id, parent_id, is_parent, is_active, 
            created_at, created_by, updated_at, updated_by
        "#,
        request.name,
        request.alias_name,
        request.cash_flow_type,
        request.ratio_type,
        request.is_variable_cost,
        request.parent_id,
        request.is_parent,
        request.is_active,
        current_user_id,
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::not_found("Account Subclassification", &id.to_string()))?;

    Ok(ApiResponse::success_with_message(
        record,
        "Account subclassification updated successfully".to_string(),
    ))
}

/// DELETE
#[instrument(skip(pool))]
pub async fn delete(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<Uuid>,
) -> ApiResult<()> {
    info!("Deleting account subclassification: {}", id);

    let current_user_id = Uuid::new_v4();

    let affected = sqlx::query!(
        "UPDATE account_subclassifications SET is_active = false, updated_by = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND is_active = true",
        current_user_id,
        id
    )
    .execute(&pool)
    .await?
    .rows_affected();

    if affected == 0 {
        return Err(AppError::not_found(
            "Account Subclassification",
            &id.to_string(),
        ));
    }

    Ok(ApiResponse::success_with_message(
        (),
        "Account subclassification deleted successfully".to_string(),
    ))
}
