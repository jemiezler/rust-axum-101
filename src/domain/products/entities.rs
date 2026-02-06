use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub created_at: String,
    pub updated_at: String,
}
