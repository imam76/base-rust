use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, Arguments, Column, PgPool, Row};
use std::collections::HashMap;
use tracing::{debug, warn};
use uuid::Uuid; // Add tracing for logging

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<u32>,
    pub search: Option<String>,
    pub filter: Option<String>,  // JSON string
    pub include: Option<String>, // comma-separated relation names
    #[allow(dead_code)]
    pub exclude: Option<String>, // JSON array
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,

    // Dynamic search fields support
    // Format: search_fields=field1,field2,field3&search_value=searchterm
    pub search_fields: Option<String>, // comma-separated field names
    pub search_value: Option<String>,  // search term for the fields
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
            search_fields: None,
            search_value: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub count: u64,
    pub page_context: PageContext,
    pub links: PaginationLinks,
    pub results: Vec<T>,
}

#[derive(Debug, Serialize)]
pub struct PageContext {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct PaginationLinks {
    pub first: String,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub last: String,
}

pub struct QueryBuilder {
    table: String,
    select_fields: Vec<String>,
    search_fields: Vec<String>,
    filterable_fields: Vec<String>,
    sortable_fields: Vec<String>,
    joins: Vec<String>,
    include_relations: HashMap<String, IncludeConfig>,
}

#[derive(Clone)]
pub struct IncludeConfig {
    pub join_clause: String,
    pub select_fields: Vec<String>,
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
            include_relations: HashMap::new(),
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

    pub fn include_relation(
        mut self,
        name: &str,
        join_clause: &str,
        select_fields: Vec<&str>,
    ) -> Self {
        self.include_relations.insert(
            name.to_string(),
            IncludeConfig {
                join_clause: join_clause.to_string(),
                select_fields: select_fields.iter().map(|s| s.to_string()).collect(),
            },
        );
        self
    }

    fn generate_pagination_links(
        &self,
        page: u32,
        total_pages: u32,
        per_page: u32,
        base_url: &str,
    ) -> PaginationLinks {
        let first = format!("{}?page=1&perPage={}", base_url, per_page);
        let last = format!("{}?page={}&perPage={}", base_url, total_pages, per_page);

        let previous = if page > 1 {
            Some(format!(
                "{}?page={}&perPage={}",
                base_url,
                page - 1,
                per_page
            ))
        } else {
            None
        };

        let next = if page < total_pages {
            Some(format!(
                "{}?page={}&perPage={}",
                base_url,
                page + 1,
                per_page
            ))
        } else {
            None
        };

        PaginationLinks {
            first,
            previous,
            next,
            last,
        }
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

        // Add static joins
        for join in &self.joins {
            query.push_str(&format!(" {}", join));
        }

        // Add dynamic includes
        if let Some(includes) = &params.include {
            for include_name in includes.split(',') {
                let include_name = include_name.trim();
                if let Some(config) = self.include_relations.get(include_name) {
                    query.push_str(&format!(" {}", config.join_clause));

                    // Add include fields to SELECT if not using *
                    if !self.select_fields.contains(&"*".to_string())
                        && !config.select_fields.is_empty()
                    {
                        query = query.replace(
                            &format!("SELECT {}", self.select_fields.join(", ")),
                            &format!(
                                "SELECT {}, {}",
                                self.select_fields.join(", "),
                                config.select_fields.join(", ")
                            ),
                        );
                    }

                    debug!("Added include join for: {}", include_name);
                }
            }
        }

        let mut conditions = Vec::new();
        let mut args = PgArguments::default();
        let mut param_count = 1;

        // Search functionality with prepared statements
        // Priority: dynamic search_fields > static search_fields
        let search_term = params.search_value.as_ref().or(params.search.as_ref());

        if let Some(search) = search_term {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);

                // Determine which fields to search in
                let (fields_to_search, _invalid_fields) =
                    if let Some(dynamic_fields) = &params.search_fields {
                        // Use dynamic fields from URL parameter
                        let requested_fields: Vec<String> = dynamic_fields
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();

                        let valid_fields: Vec<String> = requested_fields
                            .iter()
                            .filter(|field| self.search_fields.contains(field))
                            .cloned()
                            .collect();

                        let invalid_fields: Vec<String> = requested_fields
                            .iter()
                            .filter(|field| !self.search_fields.contains(field))
                            .cloned()
                            .collect();

                        debug!("Dynamic search - Requested fields: {:?}", requested_fields);
                        debug!("Dynamic search - Valid fields: {:?}", valid_fields);

                        if !invalid_fields.is_empty() {
                            warn!(
                                "Invalid search fields ignored: {:?}. Valid fields: {:?}",
                                invalid_fields, self.search_fields
                            );
                        }

                        (valid_fields, invalid_fields)
                    } else {
                        // Use static/default search fields
                        debug!("Using default search fields: {:?}", self.search_fields);
                        (self.search_fields.clone(), Vec::new())
                    };

                if !fields_to_search.is_empty() {
                    let search_conditions: Vec<String> = fields_to_search
                        .iter()
                        .map(|field| {
                            let _ = args.add(&search_pattern);
                            // Handle NULL values: field IS NOT NULL AND field ILIKE pattern
                            let condition = format!(
                                "({} IS NOT NULL AND {} ILIKE ${})",
                                field, field, param_count
                            );
                            param_count += 1;
                            condition
                        })
                        .collect();

                    let search_clause = format!("({})", search_conditions.join(" OR "));
                    conditions.push(search_clause.clone());

                    debug!("Search clause added: {}", search_clause);
                    debug!("Search pattern: {}", search_pattern);
                } else {
                    warn!(
                        "No valid search fields available for search term: {}",
                        search
                    );
                }
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
                    } else {
                        warn!("Filter field '{}' is not allowed: {:?}", key, value);
                    }
                }
            } else {
                warn!("Failed to parse filter JSON: {}", filter);
            }
        }

        // Add WHERE clause
        if !conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }

        // Sorting
        let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
        let sort_order = params.sort_order.as_deref().unwrap_or("desc");

        debug!(
            "Sort requested - field: '{}', order: '{}'",
            sort_by, sort_order
        );
        debug!("Available sortable fields: {:?}", self.sortable_fields);

        if self.sortable_fields.contains(&sort_by.to_string()) {
            let order = if sort_order.to_lowercase() == "asc" {
                "ASC"
            } else {
                "DESC"
            };
            query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
            debug!("Applied sort: {} {}", sort_by, order);
        } else {
            warn!(
                "Sort field '{}' is not allowed, falling back to default sort by 'created_at'",
                sort_by
            );
            query.push_str(" ORDER BY created_at DESC");
        }

        // Pagination with prepared statements
        let _ = args.add(per_page as i64);
        let _ = args.add(offset as i64);
        query.push_str(&format!(
            " LIMIT ${} OFFSET ${}",
            param_count,
            param_count + 1
        ));

        debug!("Built query: {}", query);
        debug!("With parameters: {:?}", args);

        (query, args, page, per_page)
    }

    pub fn build_count_query(&self, params: &QueryParams) -> (String, PgArguments) {
        let mut query = format!("SELECT COUNT(*) as total FROM {}", self.table);

        // Add static joins for count
        for join in &self.joins {
            query.push_str(&format!(" {}", join));
        }

        // Add dynamic includes for count
        if let Some(includes) = &params.include {
            for include_name in includes.split(',') {
                let include_name = include_name.trim();
                if let Some(config) = self.include_relations.get(include_name) {
                    query.push_str(&format!(" {}", config.join_clause));
                }
            }
        }

        let mut conditions = Vec::new();
        let mut args = PgArguments::default();
        let mut param_count = 1;

        // Search functionality - SAME LOGIC AS build_query (FIXED!)
        let search_term = params.search_value.as_ref().or(params.search.as_ref());

        if let Some(search) = search_term {
            if !search.is_empty() {
                let search_pattern = format!("%{}%", search);

                // Determine which fields to search in (SAME AS build_query)
                let (fields_to_search, _invalid_fields) =
                    if let Some(dynamic_fields) = &params.search_fields {
                        // Use dynamic fields from URL parameter
                        let requested_fields: Vec<String> = dynamic_fields
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();

                        let valid_fields: Vec<String> = requested_fields
                            .iter()
                            .filter(|field| self.search_fields.contains(field))
                            .cloned()
                            .collect();

                        let invalid_fields: Vec<String> = requested_fields
                            .iter()
                            .filter(|field| !self.search_fields.contains(field))
                            .cloned()
                            .collect();

                        debug!(
                            "Count query - Dynamic search - Requested fields: {:?}",
                            requested_fields
                        );
                        debug!(
                            "Count query - Dynamic search - Valid fields: {:?}",
                            valid_fields
                        );

                        (valid_fields, invalid_fields)
                    } else {
                        // Use static/default search fields
                        debug!(
                            "Count query - Using default search fields: {:?}",
                            self.search_fields
                        );
                        (self.search_fields.clone(), Vec::new())
                    };

                if !fields_to_search.is_empty() {
                    let search_conditions: Vec<String> = fields_to_search
                        .iter()
                        .map(|field| {
                            let _ = args.add(&search_pattern);
                            // Handle NULL values: field IS NOT NULL AND field ILIKE pattern
                            let condition = format!(
                                "({} IS NOT NULL AND {} ILIKE ${})",
                                field, field, param_count
                            );
                            param_count += 1;
                            condition
                        })
                        .collect();

                    let search_clause = format!("({})", search_conditions.join(" OR "));
                    conditions.push(search_clause.clone());

                    debug!("Count query - Search clause added: {}", search_clause);
                }
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

        debug!("Built count query: {}", query);
        debug!("With parameters: {:?}", args);

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
        self.execute_with_base_url(pool, params, "/api/v1/data")
            .await
    }

    pub async fn execute_with_base_url<T>(
        &self,
        pool: &PgPool,
        params: &QueryParams,
        base_url: &str,
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

        let links = self.generate_pagination_links(page, total_pages, per_page, base_url);

        Ok(PaginatedResponse {
            count: total,
            page_context: PageContext {
                page,
                per_page,
                total_pages,
            },
            links,
            results: data,
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
