use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::ConfigError,
    models::product::{CreateProduct, Product, ProductWithSku, ProductWithSlug},
};

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

pub async fn getall_products_reository(pool: &PgPool) -> Result<Vec<Product>, ConfigError> {
    let products = sqlx::query_as::<_, Product>(QUERY_PRODUCTS)
        .fetch_all(pool)
        .await
        .map_err(|_| ConfigError::InternalServerError("Error".to_string()))?;

    Ok(products)
}

const QUERY_PRODUCTS_BYID: &str = r#"
    SELECT * FROM products
    WHERE id = $1"#;

// GET /api/v1/products/:id
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

//GET /api/v1/products/slug/:slug

const QUERY_PRODUCTS_BYSLUG: &str = r#"
    SELECT id, name, slug FROM products
    WHERE slug LIKE '%' || $1 || '%'
    ORDER BY
        CASE
            WHEN slug = $1 THEN 0
            WHEN slug LIKE $1 || '-%' THEN 1
            WHEN slug LIKE $1 || '%-' THEN 2
            WHEN slug LIKE '%-' || $1 || '-%' THEN 3
            ELSE 4
        END,
        slug
"#;

pub async fn get_product_by_slug_repository(
    pool: &PgPool,
    slug: String,
) -> Result<Vec<ProductWithSlug>, ConfigError> {
    let products_slug = sqlx::query_as::<_, ProductWithSlug>(QUERY_PRODUCTS_BYSLUG)
        .bind(slug)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            ConfigError::InternalServerError(format!("Error fetching product by slug: {}", e))
        })?;

    Ok(products_slug)
}

//GET /api/v1/products/sku/:sku
const QUERY_PRODUCTS_SKU: &str = r#"
    SELECT id, name, sku FROM products
    WHERE sku ILIKE '%' || $1 || '%'
    ORDER BY
        CASE
            WHEN sku= $1 THEN 0
            WHEN sku LIKE $1 || '-%' THEN 1
            WHEN sku LIKE $1 || '%-' THEN 2
            WHEN sku LIKE '%-' || $1 || '-%' THEN 3
            ELSE 4
        END,
        sku
"#;

pub async fn get_product_by_sku_repository(
    pool: &PgPool,
    sku: String,
) -> Result<Vec<ProductWithSku>, ConfigError> {
    let products_sku = sqlx::query_as::<_, ProductWithSku>(QUERY_PRODUCTS_SKU)
        .bind(sku)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            ConfigError::InternalServerError(format!("Error fetching product by sku: {}", e))
        })?;

    Ok(products_sku)
}

//POST /api/v1/products
const QUERY_INSERT_PRODUCTS: &str = r#"
INSERT INTO products (id, sku, name, slug, description, category_id, brand_id, price, stock_quantity, status, created_at)
VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
RETURNING * "#;
pub async fn create_product_repository(
    pool: &PgPool,
    product: CreateProduct,
) -> Result<Product, ConfigError> {
    //let price_normalized = product.price.with_scale(2);
    let result = sqlx::query_as::<_, Product>(QUERY_INSERT_PRODUCTS)
        .bind(product.sku)
        .bind(product.name)
        .bind(product.slug)
        .bind(product.description)
        .bind(product.category_id)
        .bind(product.brand_id)
        .bind(product.price.with_scale(2))
        .bind(product.stock_quantity)
        .bind(product.status)
        .fetch_one(pool)
        .await;
    match &result {
        Ok(p) => println!("✅ Producto creado: {:?}", p),
        Err(e) => println!("❌ Error BD: {}", e),
    }

    result.map_err(|e| ConfigError::InternalServerError(format!("{}", e)))

    //     .map_err(|error| {
    //         ConfigError::InternalServerError(format!("Error create product: {}", error))
    //     })?;
    // Ok(product)
}
