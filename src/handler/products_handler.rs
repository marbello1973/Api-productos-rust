use crate::{
    connexion::state::AppState,
    error::ConfigError,
    models::product::Product,
    service::products_service::{get_product_by_id_service, getall_products_service},
};

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn getall_products_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, ConfigError> {
    let products = getall_products_service(&state).await?;
    println!("{:?}", products);
    Ok(Json(products))
}

pub async fn get_product_by_id_hanlder(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, ConfigError> {
    let product_id = Uuid::parse_str(&id)
        .map_err(|e| ConfigError::InvalidUuid(format!("Invalid UUID: {}: -{}", id, e)))?;

    if product_id.is_nil() {
        println!("ID not exists");
    }
    let product = get_product_by_id_service(state, product_id).await?;
    Ok(Json(product))
}

// pub async fn get_product_by_id_repository(
