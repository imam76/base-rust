use axum::{Extension, Json, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use crate::models::ResAccountSubclassification;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAccountSubclassification {
    pub code: String,
    pub name: String,
    pub alias_name: Option<String>,
    pub cash_flow_type: String,
    pub ratio_type: Option<String>,
    pub is_variable_cost: bool,
    pub account_classification_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub is_parent: bool,
    pub is_active: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

pub fn asd() {
    println!("asd");
}

pub async fn create(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(data): Json<CreateAccountSubclassification>,
) -> Result<Json<ResAccountSubclassification>, StatusCode> {
    info!(
        "Creating account subclassification with code: {}",
        data.code
    );

    let record = sqlx::query_as!(
        ResAccountSubclassification,
        r#"
            INSERT INTO account_subclassifications (
              code, name, alias_name, cash_flow_type, ratio_type, is_variable_cost, 
              account_classification_id, parent_id, is_parent, is_active, created_by, updated_by
            ) VALUES (
              $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            ) 
            RETURNING 
              id, code, name, alias_name, cash_flow_type, ratio_type, is_variable_cost, 
              is_parent, account_classification_id, parent_id, is_active, created_by, 
              updated_by, created_at, updated_at
        "#,
        data.code,
        data.name,
        data.alias_name,
        data.cash_flow_type,
        data.ratio_type,
        data.is_variable_cost,
        data.account_classification_id,
        data.parent_id,
        data.is_parent,
        data.is_active,
        Uuid::new_v4(),
        Uuid::new_v4()
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Construct the response struct manually
    // let response = ResAccountSubclassification {
    //     id: record.id,
    //     code: record.code,
    //     name: record.name,
    //     alias_name: record.alias_name,
    //     cash_flow_type: record.cash_flow_type,
    //     ratio_type: record.ratio_type,
    //     is_variable_cost: record.is_variable_cost,
    //     is_parent: record.is_parent,
    //     account_classification_id: record.account_classification_id,
    //     parent_id: record.parent_id,
    //     is_active: record.is_active,
    //     created_by: record.created_by,
    //     updated_by: record.updated_by,
    //     created_at: record.created_at,
    //     updated_at: record.updated_at,
    // };

    Ok(Json(record))
}
