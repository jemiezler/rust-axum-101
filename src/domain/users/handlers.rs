use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use super::dtos::create::CreateUserRequest;
use super::dtos::response::UserResponse;
use super::dtos::update::UpdateUserRequest;
use super::usecases;
use crate::app::state::AppState;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::types::result::DomainResult;

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Get all users", body = [UserResponse])
    )
)]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<UserResponse>>, AppError> {
    match usecases::get_all_users(&state.db).await {
        DomainResult::Ok(users) => Ok(ApiResponse::ok(
            users.into_iter().map(UserResponse::from).collect(),
        )),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
        _ => Err(AppError::internal_server_error(
            "Unexpected error".to_string(),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "Get user by ID", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn find_one_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<UserResponse>, AppError> {
    match usecases::find_one_user(&state.db, id).await {
        DomainResult::Ok(user) => Ok(ApiResponse::ok(user.into())),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Create new user", body = UserResponse)
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<ApiResponse<UserResponse>, AppError> {
    match usecases::create_user(&state.db, req).await {
        DomainResult::Ok(user) => Ok(ApiResponse::created(user.into())),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
        _ => Err(AppError::internal_server_error(
            "Unexpected error".to_string(),
        )),
    }
}

#[utoipa::path(
    put,
    path = "/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Update user", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<ApiResponse<UserResponse>, AppError> {
    match usecases::update_user(&state.db, id, req).await {
        DomainResult::Ok(user) => Ok(ApiResponse::ok(user.into())),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    responses(
        (status = 200, description = "Delete user"),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    match usecases::delete_user(&state.db, id).await {
        DomainResult::Ok(_) => Ok(ApiResponse::ok(())),
        DomainResult::NotFound => Err(AppError::not_found("User not found")),
        DomainResult::Err(e) => Err(AppError::internal_server_error(e)),
    }
}
