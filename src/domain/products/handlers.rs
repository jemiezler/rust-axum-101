use super::entities;
use super::usecases;
use crate::shared::error::AppError;
use crate::shared::extractors::ValidatedPath;
use crate::shared::response::ApiResponse;
use crate::shared::types::result::DomainResult;

pub async fn find_one_product(
    ValidatedPath(product_id): ValidatedPath<i32>,
) -> Result<ApiResponse<entities::Product>, AppError> {
    let product_id_int = product_id.abs();
    match usecases::find_one_product(product_id_int) {
        DomainResult::Ok(product) => Ok(ApiResponse::ok(product)),
        DomainResult::NotFound => Err(AppError::not_found("Product not found")),
        DomainResult::Err(e) => Err(AppError::bad_request(e)),
    }
}
