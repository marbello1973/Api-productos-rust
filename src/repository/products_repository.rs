use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::ConfigError, models::product::Product};

const QUERY_PRODUCTS: &str = r#"
    SELECT 
        id, 
        sku, 
        name, 
        slug, 
        description, 
        category_id, 
        brand_id, 
        price, 
        stock_quantity, 
        status,
        created_at
    FROM products"#;

const QUERY_PRODUCTS_BYID: &str = r#"
    SELECT * FROM products
    WHERE id = $1"#;

pub async fn getall_products_reository(pool: &PgPool) -> Result<Vec<Product>, ConfigError> {
    let products = sqlx::query_as::<_, Product>(QUERY_PRODUCTS)
        .fetch_all(pool)
        .await
        .map_err(|_| ConfigError::InternalServerError("Error".to_string()))?;

    Ok(products)
}

pub async fn get_product_by_id_repository(
    pool: &PgPool,
    product_id: Uuid,
) -> Result<Option<Product>, ConfigError> {
    let product = sqlx::query_as::<_, Product>(QUERY_PRODUCTS_BYID)
        .bind(product_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            ConfigError::InternalServerError(format!("Error fetching product by ID: {}", e))
        })?;

    Ok(Some(product))
}
