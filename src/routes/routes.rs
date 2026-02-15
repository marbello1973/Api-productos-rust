use crate::connexion::state::AppState;
use crate::handler::brand_handler::getall_brand_handler;
use crate::handler::{
    category_handler::getall_category_handler,
    products_handler::{get_product_by_id_hanlder, getall_products_handler},
};
use axum::{Router, routing::get};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/products", get(getall_products_handler))
        .route("/api/v1/products/{id}", get(get_product_by_id_hanlder))
        .route("/api/v1/categories", get(getall_category_handler))
        .route("/api/v1/brands", get(getall_brand_handler))
        .with_state(state)
}
