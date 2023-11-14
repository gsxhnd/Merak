use super::AppState;
use crate::handler;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(handler::tag::get_tag)
                .post(handler::tag::add_tag)
                .put(handler::tag::upd_tag)
                .delete(handler::tag::del_tag),
        )
        .with_state(state)
}
