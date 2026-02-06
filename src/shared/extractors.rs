use super::error::AppError;
use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

pub struct ValidatedPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(ValidatedPath(value.0)),
            Err(rejection) => {
                let err_msg = rejection.body_text();
                Err(AppError::bad_request(format!("{}", err_msg)))
            }
        }
    }
}
