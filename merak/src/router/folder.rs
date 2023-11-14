use super::AppState;
use crate::handler;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler::folder::get_folders))
        .route("/", post(handler::folder::add_folders))
        .route("/", put(handler::folder::upd_folder))
        .route("/", delete(handler::folder::del_folder))
        .with_state(state)
}
