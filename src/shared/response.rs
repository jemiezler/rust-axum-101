use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Standard API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    status_code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Fallback to 500 if status code is invalid, though u16 should be fine mostly
        let status =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a new generic response
    pub fn new(status: StatusCode, message: String, data: Option<T>) -> Self {
        Self {
            status_code: status.as_u16(),
            message,
            data,
        }
    }

    /// 200 OK success response
    pub fn ok(data: T) -> Self {
        Self::new(StatusCode::OK, "success".to_string(), Some(data))
    }

    /// 201 Created response
    pub fn created(data: T) -> Self {
        Self::new(StatusCode::CREATED, "created".to_string(), Some(data))
    }
}

impl ApiResponse<()> {
    /// 200 OK with no data
    pub fn ok_empty() -> Self {
        Self::new(StatusCode::OK, "success".to_string(), None)
    }
}
