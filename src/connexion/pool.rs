use sqlx::postgres::PgPoolOptions;
use std::env;

const URL: &str = "postgres://postgres:postgres@localhost:5432/register";
pub async fn connection_pool(database_url: Option<&str>) -> Result<sqlx::PgPool, sqlx::Error> {
    let url = database_url
        .map(|s| s.to_string())
        .or_else(|| {
            dotenv::dotenv().ok();
            env::var("DATABASE_URL").ok()
        })
        .unwrap_or_else(|| URL.to_string());
    PgPoolOptions::new().max_connections(5).connect(&url).await
}
