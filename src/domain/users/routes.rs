use axum::Router;
use axum::routing::get;

use super::handlers;
use crate::app::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_all_users).post(handlers::create_user),
        )
        .route(
            "/{id}",
            get(handlers::find_one_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
}
