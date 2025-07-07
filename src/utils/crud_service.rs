use axum::{Extension, extract::{Query, State, Path}, Json, response::{Response, IntoResponse}, http::StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use sqlx::{postgres::PgArguments, Arguments, Row, Column};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::{
    models::{AppState, AuthenticatedUser},
    utils::query_builder::{QueryBuilder, QueryParams, PaginatedResponse},
    AppError,
};

pub struct CrudService;

impl CrudService {
    // Generic GET handler with include relations support
    pub async fn get_list_with_includes<T>(
        table: &str,
        select_fields: Vec<&str>,
        searchable_fields: Vec<&str>,
        filterable_fields: Vec<&str>,
        sortable_fields: Vec<&str>,
        joins: Vec<&str>,
        includes: Vec<(&str, &str, Vec<&str>)>, // (name, join_clause, select_fields)
        query: Query<QueryParams>,
        state: State<AppState>,
        _auth: Option<Extension<AuthenticatedUser>>,
    ) -> Result<Json<PaginatedResponse<T>>, AppError>
    where
        T: DeserializeOwned + Send + Unpin,
    {
        let mut builder = QueryBuilder::new(table)
            .select(select_fields)
            .searchable(searchable_fields)
            .filterable(filterable_fields)
            .sortable(sortable_fields);

        // Add static joins
        for join in joins {
            builder = builder.join(join);
        }

        // Add include relations
        for (name, join_clause, include_fields) in includes {
            builder = builder.include_relation(name, join_clause, include_fields);
        }

        let result = builder
            .execute(&state.db, &query.0)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Json(result))
    }

    // Generic GET handler for any table
    pub async fn get_list<T>(
        table: &str,
        select_fields: Vec<&str>,
        searchable_fields: Vec<&str>,
        filterable_fields: Vec<&str>,
        sortable_fields: Vec<&str>,
        joins: Vec<&str>,
        query: Query<QueryParams>,
        state: State<AppState>,
        _auth: Option<Extension<AuthenticatedUser>>, // Optional auth
    ) -> Result<Json<PaginatedResponse<T>>, AppError>
    where
        T: DeserializeOwned + Send + Unpin,
    {
        let mut builder = QueryBuilder::new(table)
            .select(select_fields)
            .searchable(searchable_fields)
            .filterable(filterable_fields)
            .sortable(sortable_fields);

        // Add joins
        for join in joins {
            builder = builder.join(join);
        }

        let result = builder
            .execute(&state.db, &query.0)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(Json(result))
    }

    // Generic GET by ID handler
    pub async fn get_by_id<T>(
        table: &str,
        select_fields: Vec<&str>,
        joins: Vec<&str>,
        id: Path<Uuid>,
        state: State<AppState>,
        _auth: Option<Extension<AuthenticatedUser>>,
    ) -> Result<Json<T>, AppError>
    where
        T: DeserializeOwned + Send + Unpin,
    {
        let select_clause = select_fields.join(", ");
        let joins_clause = joins.join(" ");
        
        let query = format!(
            "SELECT {} FROM {} {} WHERE {}.id = $1",
            select_clause, table, joins_clause, table
        );

        let row = sqlx::query(&query)
            .bind(*id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound { id: *id })?;

        // Convert row to JSON then to T
        let mut json_obj = serde_json::Map::new();
        for column in row.columns() {
            let column_name = column.name();
            let value: serde_json::Value = Self::row_value_to_json(&row, column_name);
            json_obj.insert(column_name.to_string(), value);
        }

        let item: T = serde_json::from_value(serde_json::Value::Object(json_obj))
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        Ok(Json(item))
    }

    // Generic CREATE handler with dynamic field handling
    pub async fn create<T, C>(
        table: &str,
        create_data: Json<C>,
        state: State<AppState>,
        auth: Extension<AuthenticatedUser>,
    ) -> Result<Json<T>, AppError>
    where
        T: DeserializeOwned + Send + Unpin,
        C: Serialize,
    {
        // Convert create data to HashMap for dynamic insert
        let data_value = serde_json::to_value(&create_data.0)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        
        let data_map: HashMap<String, serde_json::Value> = serde_json::from_value(data_value)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut args = PgArguments::default();

        // Add standard fields
        columns.push("id".to_string());
        placeholders.push("$1".to_string());
        let new_id = Uuid::new_v4();
        let _ = args.add(new_id);

        columns.push("created_by".to_string());
        placeholders.push("$2".to_string());
        let _ = args.add(auth.user_id());

        columns.push("updated_by".to_string());
        placeholders.push("$3".to_string());
        let _ = args.add(auth.user_id());

        // Add dynamic fields
        let mut param_count = 4;
        for (key, value) in data_map {
            if key != "id" && key != "created_by" && key != "updated_by" && key != "created_at" && key != "updated_at" {
                columns.push(key.clone());
                placeholders.push(format!("${}", param_count));
                
                match value {
                    serde_json::Value::String(s) => {
                        let _ = args.add(s);
                    }
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            let _ = args.add(i);
                        } else if let Some(f) = n.as_f64() {
                            let _ = args.add(f);
                        } else {
                            let _ = args.add(0i64); // fallback
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        let _ = args.add(b);
                    }
                    serde_json::Value::Null => {
                        let _ = args.add(Option::<String>::None);
                    }
                    _ => {
                        let _ = args.add(value.to_string()); // Fallback to string
                    }
                }
                param_count += 1;
            }
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );

        let row = sqlx::query_with(&query, args)
            .fetch_one(&state.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Convert to T
        let mut json_obj = serde_json::Map::new();
        for column in row.columns() {
            let column_name = column.name();
            let value = Self::row_value_to_json(&row, column_name);
            json_obj.insert(column_name.to_string(), value);
        }

        let item: T = serde_json::from_value(serde_json::Value::Object(json_obj))
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        Ok(Json(item))
    }

    // Generic UPDATE handler with dynamic field handling
    pub async fn update<T, U>(
        table: &str,
        id: Path<Uuid>,
        update_data: Json<U>,
        state: State<AppState>,
        auth: Extension<AuthenticatedUser>,
    ) -> Result<Json<T>, AppError>
    where
        T: DeserializeOwned + Send + Unpin,
        U: Serialize,
    {
        // Convert update data to HashMap for dynamic update
        let data_value = serde_json::to_value(&update_data.0)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        
        let data_map: HashMap<String, serde_json::Value> = serde_json::from_value(data_value)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        if data_map.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        let mut set_clauses = Vec::new();
        let mut args = PgArguments::default();

        // Add standard fields
        set_clauses.push("updated_by = $1".to_string());
        let _ = args.add(auth.user_id());

        set_clauses.push("updated_at = NOW()".to_string());

        // Add dynamic fields
        let mut param_count = 2;
        for (key, value) in data_map {
            if key != "id" && key != "created_by" && key != "updated_by" && key != "created_at" && key != "updated_at" {
                set_clauses.push(format!("{} = ${}", key, param_count));
                
                match value {
                    serde_json::Value::String(s) => {
                        let _ = args.add(s);
                    }
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            let _ = args.add(i);
                        } else if let Some(f) = n.as_f64() {
                            let _ = args.add(f);
                        } else {
                            let _ = args.add(0i64); // fallback
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        let _ = args.add(b);
                    }
                    serde_json::Value::Null => {
                        let _ = args.add(Option::<String>::None);
                    }
                    _ => {
                        let _ = args.add(value.to_string()); // Fallback to string
                    }
                }
                param_count += 1;
            }
        }

        // Add ID for WHERE clause
        let _ = args.add(*id);

        let query = format!(
            "UPDATE {} SET {} WHERE id = ${} RETURNING *",
            table,
            set_clauses.join(", "),
            param_count
        );

        let row = sqlx::query_with(&query, args)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound { id: *id })?;

        // Convert to T
        let mut json_obj = serde_json::Map::new();
        for column in row.columns() {
            let column_name = column.name();
            let value = Self::row_value_to_json(&row, column_name);
            json_obj.insert(column_name.to_string(), value);
        }

        let item: T = serde_json::from_value(serde_json::Value::Object(json_obj))
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        Ok(Json(item))
    }

    // Generic DELETE handler
    pub async fn delete(
        table: &str,
        id: Path<Uuid>,
        state: State<AppState>,
        _auth: Extension<AuthenticatedUser>,
    ) -> Result<Response, AppError> {
        let query = format!("DELETE FROM {} WHERE id = $1", table);
        
        let result = sqlx::query(&query)
            .bind(*id)
            .execute(&state.db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound { id: *id });
        }

        Ok((StatusCode::NO_CONTENT).into_response())
    }

    // Helper function to convert row values to JSON
    fn row_value_to_json(row: &sqlx::postgres::PgRow, column_name: &str) -> serde_json::Value {
        let column = row.columns()
            .iter()
            .find(|c| c.name() == column_name)
            .unwrap();

        // Use ordinal to access column
        let ordinal = column.ordinal();
        
        // Try different types based on common PostgreSQL types
        if let Ok(value) = row.try_get::<String, _>(ordinal) {
            return serde_json::Value::String(value);
        }
        if let Ok(value) = row.try_get::<Uuid, _>(ordinal) {
            return serde_json::Value::String(value.to_string());
        }
        if let Ok(value) = row.try_get::<i32, _>(ordinal) {
            return serde_json::Value::Number(value.into());
        }
        if let Ok(value) = row.try_get::<i64, _>(ordinal) {
            return serde_json::Value::Number(value.into());
        }
        if let Ok(value) = row.try_get::<bool, _>(ordinal) {
            return serde_json::Value::Bool(value);
        }
        if let Ok(value) = row.try_get::<DateTime<Utc>, _>(ordinal) {
            return serde_json::Value::String(value.to_rfc3339());
        }
        if let Ok(value) = row.try_get::<f64, _>(ordinal) {
            if let Some(number) = serde_json::Number::from_f64(value) {
                return serde_json::Value::Number(number);
            }
        }
        if let Ok(value) = row.try_get::<serde_json::Value, _>(ordinal) {
            return value;
        }
        
        serde_json::Value::Null
    }
}
