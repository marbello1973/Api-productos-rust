use uuid::Uuid;

use crate::{
    connexion::state::AppState,
    error::ConfigError,
    models::product::{CreateProduct, Product, ProductWithSku, ProductWithSlug},
    repository::products_repository::{
        create_product_repository, get_product_by_id_repository, get_product_by_sku_repository,
        get_product_by_slug_repository, getall_products_reository,
    },
};

pub async fn getall_products_service(state: &AppState) -> Result<Vec<Product>, ConfigError> {
    let products = getall_products_reository(&state.db).await?;
    Ok(products)
}

pub async fn get_product_by_id_service(
    state: AppState,
    product_id: Uuid,
) -> Result<Product, ConfigError> {
    let product = get_product_by_id_repository(&state.db, product_id).await?;
    match product {
        Some(p) => Ok(p),
        None => Err(ConfigError::NotFound(format!(
            "Product with ID {} not found",
            product_id
        ))),
    }
}

pub async fn get_product_by_slug_service(
    state: AppState,
    slug: String,
) -> Result<Vec<ProductWithSlug>, ConfigError> {
    let product = get_product_by_slug_repository(&state.db, slug.clone()).await?;
    if product.is_empty() {
        return Err(ConfigError::NotFound("Not product".to_string()));
    }
    Ok(product)
}

pub async fn get_product_by_sku_service(
    state: AppState,
    sku: String,
) -> Result<Vec<ProductWithSku>, ConfigError> {
    match get_product_by_sku_repository(&state.db, sku).await {
        Ok(ok) => Ok(ok),
        Err(_) => Err(ConfigError::NotFound("Not products sku".to_string())),
    }
    // let product_sku = get_product_by_sku_repository(&state.db, slug.clone()).await?;
    // if product_sku.is_empty() {
    // return Err(ConfigError::NotFound("Not product".to_string()));
    // }
    // Ok(product_sku)
}

pub async fn create_product_service(
    state: AppState,
    product: CreateProduct,
) -> Result<Product, ConfigError> {
    match create_product_repository(&state.db, product).await {
        Ok(ok) => Ok(ok),
        Err(_) => Err(ConfigError::InternalServerError(format!("Error"))),
    }
}
