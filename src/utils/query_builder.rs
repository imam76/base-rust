use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, Arguments, Column, PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub filter: Option<String>, // JSON string
    #[allow(dead_code)]
    pub include: Option<String>, // JSON array
    #[allow(dead_code)]
    pub exclude: Option<String>, // JSON array
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            search: None,
            filter: None,
            include: None,
            exclude: None,
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

pub struct QueryBuilder {
    table: String,
    select_fields: Vec<String>,
    search_fields: Vec<String>,
    filterable_fields: Vec<String>,
    sortable_fields: Vec<String>,
    joins: Vec<String>,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            select_fields: vec!["*".to_string()],
            search_fields: Vec::new(),
            filterable_fields: Vec::new(),
            sortable_fields: Vec::new(),
            joins: Vec::new(),
        }
    }

    pub fn select(mut self, fields: Vec<&str>) -> Self {
        self.select_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn searchable(mut self, fields: Vec<&str>) -> Self {
        self.search_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn filterable(mut self, fields: Vec<&str>) -> Self {
        self.filterable_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn sortable(mut self, fields: Vec<&str>) -> Self {
        self.sortable_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn join(mut self, join_clause: &str) -> Self {
        self.joins.push(join_clause.to_string());
        self
    }

    pub fn build_query(&self, params: &QueryParams) -> (String, PgArguments, u32, u32) {
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(10).min(100); // Max 100 per page
        let offset = (page - 1) * per_page;

        let mut query = format!(
            "SELECT {} FROM {}",
            self.select_fields.join(", "),
            self.table
        );

        // Add joins
        for join in &self.joins {
            query.push_str(&format!(" {}", join));
        }

        let mut conditions = Vec::new();
        let mut args = PgArguments::default();
        let mut param_count = 1;

        // Search functionality with prepared statements
        if let Some(search) = &params.search {
            if !search.is_empty() && !self.search_fields.is_empty() {
                let search_pattern = format!("%{}%", search);
                let search_conditions: Vec<String> = self
                    .search_fields
                    .iter()
                    .map(|field| {
                        let _ = args.add(&search_pattern);
                        let condition = format!("{} ILIKE ${}", field, param_count);
                        param_count += 1;
                        condition
                    })
                    .collect();
                conditions.push(format!("({})", search_conditions.join(" OR ")));
            }
        }

        // Filter functionality with prepared statements
        if let Some(filter) = &params.filter {
            if let Ok(filters) = serde_json::from_str::<HashMap<String, serde_json::Value>>(filter)
            {
                for (key, value) in filters {
                    if self.filterable_fields.contains(&key) {
                        match value {
                            serde_json::Value::String(s) => {
                                let _ = args.add(s);
                                conditions.push(format!("{} = ${}", key, param_count));
                                param_count += 1;
                            }
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    let _ = args.add(i);
                                    conditions.push(format!("{} = ${}", key, param_count));
                                    param_count += 1;
                                } else if let Some(f) = n.as_f64() {
                                    let _ = args.add(f);
                                    conditions.push(format!("{} = ${}", key, param_count));
                                    param_count += 1;
                                }
                            }
                            serde_json::Value::Bool(b) => {
                                let _ = args.add(b);
                                conditions.push(format!("{} = ${}", key, param_count));
                                param_count += 1;
                            }
                            _ => {} // Skip complex types
                        }
                    }
                }
            }
        }

        // Add WHERE clause
        if !conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        // Sorting
        let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
        let sort_order = params.sort_order.as_deref().unwrap_or("desc");

        if self.sortable_fields.contains(&sort_by.to_string()) {
            let order = if sort_order.to_lowercase() == "asc" {
                "ASC"
            } else {
                "DESC"
            };
            query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
        }

        // Pagination with prepared statements
        let _ = args.add(per_page as i64);
        let _ = args.add(offset as i64);
        query.push_str(&format!(
            " LIMIT ${} OFFSET ${}",
            param_count,
            param_count + 1
        ));

        (query, args, page, per_page)
    }

    pub fn build_count_query(&self, params: &QueryParams) -> (String, PgArguments) {
        let mut query = format!("SELECT COUNT(*) as total FROM {}", self.table);

        // Add joins for count (without SELECT part)
        for join in &self.joins {
            query.push_str(&format!(" {}", join));
        }

        let mut conditions = Vec::new();
        let mut args = PgArguments::default();
        let mut param_count = 1;

        // Search functionality (same as main query)
        if let Some(search) = &params.search {
            if !search.is_empty() && !self.search_fields.is_empty() {
                let search_pattern = format!("%{}%", search);
                let search_conditions: Vec<String> = self
                    .search_fields
                    .iter()
                    .map(|field| {
                        let _ = args.add(&search_pattern);
                        let condition = format!("{} ILIKE ${}", field, param_count);
                        param_count += 1;
                        condition
                    })
                    .collect();
                conditions.push(format!("({})", search_conditions.join(" OR ")));
            }
        }

        // Filter functionality (same as main query)
        if let Some(filter) = &params.filter {
            if let Ok(filters) = serde_json::from_str::<HashMap<String, serde_json::Value>>(filter)
            {
                for (key, value) in filters {
                    if self.filterable_fields.contains(&key) {
                        match value {
                            serde_json::Value::String(s) => {
                                let _ = args.add(s);
                                conditions.push(format!("{} = ${}", key, param_count));
                                param_count += 1;
                            }
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    let _ = args.add(i);
                                    conditions.push(format!("{} = ${}", key, param_count));
                                    param_count += 1;
                                } else if let Some(f) = n.as_f64() {
                                    let _ = args.add(f);
                                    conditions.push(format!("{} = ${}", key, param_count));
                                    param_count += 1;
                                }
                            }
                            serde_json::Value::Bool(b) => {
                                let _ = args.add(b);
                                conditions.push(format!("{} = ${}", key, param_count));
                                param_count += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if !conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        (query, args)
    }

    pub async fn execute<T>(
        &self,
        pool: &PgPool,
        params: &QueryParams,
    ) -> Result<PaginatedResponse<T>, sqlx::Error>
    where
        T: for<'r> serde::de::Deserialize<'r> + Send + Unpin,
    {
        let (count_query, count_args) = self.build_count_query(params);
        let (query, args, page, per_page) = self.build_query(params);

        // Execute count query
        let total_row = sqlx::query_with(&count_query, count_args)
            .fetch_one(pool)
            .await?;
        let total: i64 = total_row.get("total");
        let total = total as u64;

        // Execute main query
        let rows = sqlx::query_with(&query, args).fetch_all(pool).await?;

        // Convert rows to JSON and then deserialize to T
        let mut data = Vec::new();
        for row in rows {
            let mut json_obj = serde_json::Map::new();
            for column in row.columns() {
                let column_name = column.name();
                let value: serde_json::Value = self.row_value_to_json(&row, column_name);
                json_obj.insert(column_name.to_string(), value);
            }

            let item: T =
                serde_json::from_value(serde_json::Value::Object(json_obj)).map_err(|e| {
                    sqlx::Error::ColumnDecode {
                        index: e.to_string(),
                        source: Box::new(e),
                    }
                })?;
            data.push(item);
        }

        let total_pages = (total as f64 / per_page as f64).ceil() as u32;

        Ok(PaginatedResponse {
            data,
            pagination: PaginationMeta {
                current_page: page,
                per_page,
                total,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    fn row_value_to_json(
        &self,
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> serde_json::Value {
        // Try to get the value based on the column type
        let column = row
            .columns()
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
