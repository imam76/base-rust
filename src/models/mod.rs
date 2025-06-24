use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResAccountSubclassification {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}
