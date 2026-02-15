use sqlx::PgPool;

use crate::{error::ConfigError, models::product::Brand};

const QUERY_BRAND: &str = r#"
    SELECT id, name, slug
    FROM brands"#;

pub async fn getall_brand_repository(pool: &PgPool) -> Result<Vec<Brand>, ConfigError> {
    let brand = sqlx::query_as::<_, Brand>(QUERY_BRAND)
        .fetch_all(pool)
        .await
        .map_err(|_| ConfigError::InternalServerError("Error".to_string()))?;

    Ok(brand)
}
