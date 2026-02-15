use crate::{
    connexion::state::AppState, error::ConfigError, models::product::Brand,
    service::brand_service::getall_brand_service,
};
use axum::{extract::State, Json};

pub async fn getall_brand_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Brand>>, ConfigError> {
    let brands = getall_brand_service(&state).await?;

    Ok(Json(brands))
}
