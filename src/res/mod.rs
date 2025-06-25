// src/response/mod.rs
use axum::Json;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
    pub request_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
    pub request_id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            success: true,
            data,
            message: None,
            request_id: Uuid::new_v4(), // In real app, get from middleware
            timestamp: chrono::Utc::now(),
        })
    }

    pub fn success_with_message(data: T, message: String) -> Json<Self> {
        Json(Self {
            success: true,
            data,
            message: Some(message),
            request_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        })
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, per_page: u32, total: u64) -> Json<Self> {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;

        Json(Self {
            success: true,
            data,
            pagination: PaginationInfo {
                page,
                per_page,
                total,
                total_pages,
            },
            request_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        })
    }
}

// Convenience type aliases
pub type ApiResult<T> = Result<Json<ApiResponse<T>>, crate::errors::AppError>;
pub type PaginatedResult<T> = Result<Json<PaginatedResponse<T>>, crate::errors::AppError>;
