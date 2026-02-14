use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::BigDecimal, FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub price: BigDecimal,
    pub stock_quantity: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Review {
    pub id: Uuid,
    pub product_id: Uuid,
    pub rating: i32,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBrand {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub price: BigDecimal,
    pub stock_quantity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReview {
    pub product_id: Uuid,
    pub rating: i32,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
    pub category_id: Option<Option<Uuid>>,
    pub brand_id: Option<Option<Uuid>>,
    pub price: Option<BigDecimal>,
    pub stock_quantity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWithRelations {
    #[serde(flatten)]
    pub product: Product,
    pub category: Option<Category>,
    pub brand: Option<Brand>,
    pub reviews: Option<Vec<Review>>,
}
