use axum::Json;
use axum::extract::{Path, State};

use super::entities::{CreateUserRequest, UpdateUserRequest, User};
use super::usecases;
use crate::app::state::AppState;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::types::result::DomainResult;

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<User>>, AppError> {
    match usecases::get_all_users(&state.db).await {
        DomainResult::Ok(users) => Ok(ApiResponse::ok(users)),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
        _ => Err(AppError::internal_server_error(
            "Unexpected error".to_string(),
        )),
    }
}

pub async fn find_one_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<User>, AppError> {
    match usecases::find_one_user(&state.db, id).await {
        DomainResult::Ok(user) => Ok(ApiResponse::ok(user)),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<ApiResponse<User>, AppError> {
    match usecases::create_user(&state.db, req).await {
        DomainResult::Ok(user) => Ok(ApiResponse::created(user)),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
        _ => Err(AppError::internal_server_error(
            "Unexpected error".to_string(),
        )),
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<ApiResponse<User>, AppError> {
    match usecases::update_user(&state.db, id, req).await {
        DomainResult::Ok(user) => Ok(ApiResponse::ok(user)),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    match usecases::delete_user(&state.db, id).await {
        DomainResult::Ok(_) => Ok(ApiResponse::ok(())),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}
