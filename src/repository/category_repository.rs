use sqlx::PgPool;

use crate::{error::ConfigError, models::product::Category};

const QUERY_CATEGORY: &str = r#"
    SELECT id, name, slug 
    FROM categories"#;

pub async fn getall_category_repository(pool: &PgPool) -> Result<Vec<Category>, ConfigError> {
    let category = sqlx::query_as::<_, Category>(QUERY_CATEGORY)
        .fetch_all(pool)
        .await
        .map_err(|_| ConfigError::InternalServerError("Error".to_string()))?;

    Ok(category)
}
