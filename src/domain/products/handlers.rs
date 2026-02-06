use uuid::Uuid;

use super::entities;
use super::usecases;
use crate::shared::error::AppError;
use crate::shared::extractors::ValidatedPath;
use crate::shared::response::ApiResponse;
use crate::shared::types::result::DomainResult;

pub async fn find_one_product(
    ValidatedPath(product_id): ValidatedPath<Uuid>,
) -> Result<ApiResponse<entities::Product>, AppError> {
    match usecases::find_one_product(product_id) {
        DomainResult::Ok(product) => Ok(ApiResponse::ok(product)),
        DomainResult::NotFound => Err(AppError::not_found("Product not found")),
        DomainResult::Err(e) => Err(AppError::bad_request(e)),
    }
}
