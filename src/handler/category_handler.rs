use crate::{
    connexion::state::AppState, error::ConfigError, models::product::Category,
    service::category_service::getall_category_service,
};
use axum::{extract::State, Json};

pub async fn getall_category_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, ConfigError> {
    let categories = getall_category_service(&state).await?;

    Ok(Json(categories))
}
