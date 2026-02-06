use crate::app::api_doc::ApiDoc;
use crate::app::middleware::trace;
use crate::app::state::AppState;
use crate::domain::products;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/products", products::routes::router())
        .nest("/users", crate::domain::users::routes::router())
        .layer(trace::global_trace_layer())
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
