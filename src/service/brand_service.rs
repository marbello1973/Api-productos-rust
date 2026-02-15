use crate::{
    connexion::state::AppState, error::ConfigError, models::product::Brand,
    repository::brand_repository::getall_brand_repository,
};

pub async fn getall_brand_service(state: &AppState) -> Result<Vec<Brand>, ConfigError> {
    let brand = getall_brand_repository(&state.db).await?;
    Ok(brand)
}
