use crate::app::middleware::trace;
use crate::app::state::AppState;
use crate::domain::products;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/products", products::routes::router())
        .nest("/users", crate::domain::users::routes::router())
        .layer(trace::global_trace_layer())
}
