use crate::app::state::AppState;

use super::handlers;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/{product_id}", get(handlers::find_one_product))
}
