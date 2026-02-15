use crate::{
    connexion::state::AppState,
    error::ConfigError,
    models::product::{CreateProduct, Product, ProductWithSku, ProductWithSlug},
    service::products_service::{
        create_product_service, get_product_by_id_service, get_product_by_sku_service,
        get_product_by_slug_service, getall_products_service,
    },
};

use axum::{
    Json,
    extract::{Path, State},
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

pub async fn get_product_by_slug_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<ProductWithSlug>>, ConfigError> {
    let products = get_product_by_slug_service(state, slug).await?;
    Ok(Json(products))
}

pub async fn get_product_by_sku_handler(
    State(state): State<AppState>,
    Path(sku): Path<String>,
) -> Result<Json<Vec<ProductWithSku>>, ConfigError> {
    let products_sku = get_product_by_sku_service(state, sku).await?;
    Ok(Json(products_sku))
}

pub async fn create_product_handler(
    State(state): State<AppState>,
    Json(product): Json<CreateProduct>,
) -> Result<Json<Product>, ConfigError> {
    let product = create_product_service(state, product).await?;
    Ok(Json(product))
}
