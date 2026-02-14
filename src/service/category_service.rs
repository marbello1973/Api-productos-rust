use crate::{
    connexion::state::AppState, error::ConfigError, models::product::Category,
    repository::category_repository::getall_category_repository,
};

pub async fn getall_category_service(state: &AppState) -> Result<Vec<Category>, ConfigError> {
    let category = getall_category_repository(&state.db).await?;
    Ok(category)
}
