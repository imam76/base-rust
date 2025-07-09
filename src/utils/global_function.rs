use regex::Regex;
use sqlx::{Error, PgPool};
use tracing::info;

pub async fn generate_code(name: String, table_name: &str, pool: &PgPool) -> Result<String, Error> {
    // example ASEP IMAM NAWAWI -> AIM-00001
    let prefix = generate_prefix(&name);
    let next_number = get_next_sequence_number(&prefix, table_name, pool).await?;
    let formatted_code = format!("{}-{:05}", prefix, next_number);
    Ok(formatted_code)
}

fn generate_prefix(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
}

async fn get_next_sequence_number(
    prefix: &str,
    table_name: &str,
    pool: &PgPool,
) -> Result<i32, Error> {
    let query = format!(
        "SELECT code FROM {} WHERE code LIKE $1 ORDER BY code DESC LIMIT 1",
        table_name
    );

    let pattern = format!("{}-%", prefix);
    let result: Option<(String,)> = sqlx::query_as(&query)
        .bind(&pattern)
        .fetch_optional(pool)
        .await?;

    match result {
        Some((last_code,)) => {
            // Extract number from code like "AIM-00001"
            if let Some(number) = extract_number_from_code(&last_code) {
                Ok(number + 1)
            } else {
                Ok(1) // If can't parse, start from 1
            }
        }
        None => Ok(1), // No existing codes, start from 1
    }
}

fn extract_number_from_code(code: &str) -> Option<i32> {
    let re = Regex::new(r"-(\d+)$").ok()?;
    let caps = re.captures(code)?;
    caps.get(1)?.as_str().parse().ok()
}

// Helper function for testing without database
// pub fn generate_prefix_only(name: &str) -> String {
//     generate_prefix(name)
// }

#[derive(Debug)]
pub struct CodeRequest {
    pub text: String,
    pub code: Option<String>,
}

// Ubah function signature jadi lebih simple
pub async fn determine_code(
    request: CodeRequest,
    table: &str,
    pool: &PgPool,
) -> Result<String, Error> {
    if let Some(provided_code) = &request.code {
        if !provided_code.trim().is_empty() {
            return Ok(provided_code.trim().to_string());
        }
    }

    // Use existing generate_code function
    let full_name = format!("{}", request.text).trim().to_string();

    generate_code(full_name, table, pool).await.map_err(|e| {
        info!("Error generating code: {}", e);
        Error::from(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_prefix() {
        assert_eq!(generate_prefix("ASEP IMAM NAWAWI"), "AIN");
        assert_eq!(generate_prefix("John Doe"), "JD");
        assert_eq!(generate_prefix("Single"), "S");
        assert_eq!(generate_prefix("multiple word test case"), "MWTC");
    }

    #[test]
    fn test_extract_number_from_code() {
        assert_eq!(extract_number_from_code("AIM-00001"), Some(1));
        assert_eq!(extract_number_from_code("JD-00123"), Some(123));
        assert_eq!(extract_number_from_code("INVALID"), None);
    }
}
