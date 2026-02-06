use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Unauthorized,
    Forbidden,
    Conflict(String),
    Internal(anyhow::Error),
}

/// JSON error response body
#[derive(Serialize)]
struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => json_error(StatusCode::BAD_REQUEST, msg),

            AppError::NotFound(msg) => json_error(StatusCode::NOT_FOUND, msg),

            AppError::Unauthorized => json_error(StatusCode::UNAUTHORIZED, "unauthorized".into()),

            AppError::Forbidden => json_error(StatusCode::FORBIDDEN, "forbidden".into()),

            AppError::Conflict(msg) => json_error(StatusCode::CONFLICT, msg),

            AppError::Internal(err) => {
                // Log once, centrally
                error!(error = ?err, "internal server error");

                json_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".into(),
                )
            }
        }
    }
}

/// Helper to build JSON error responses
fn json_error(status_code: StatusCode, message: String) -> Response {
    (
        status_code,
        Json(ErrorResponse {
            status_code: status_code.as_u16(),
            message,
        }),
    )
        .into_response()
}

/// Enable `?` operator everywhere
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}

/// Optional helpers (nice ergonomics)
impl AppError {
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        AppError::BadRequest(msg.into())
    }

    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        AppError::NotFound(msg.into())
    }

    pub fn conflict<T: Into<String>>(msg: T) -> Self {
        AppError::Conflict(msg.into())
    }

    pub fn internal_server_error<T: std::fmt::Display>(msg: T) -> Self {
        AppError::Internal(anyhow::anyhow!("{}", msg))
    }
}
