use uuid::Uuid;

use crate::{
    connexion::state::AppState,
    error::ConfigError,
    models::product::Product,
    repository::products_repository::{get_product_by_id_repository, getall_products_reository},
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
    // Ok(product)
}

// pub async fn get_product_by_id_repository(
