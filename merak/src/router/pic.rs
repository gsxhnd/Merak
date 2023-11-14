use super::AppState;
use crate::handler;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler::pic::get_pics))
        .route("/", post(handler::pic::add_pics))
        .route("/", put(handler::pic::upd_pic))
        .route("/", delete(handler::pic::del_pic))
        .with_state(state)
}
