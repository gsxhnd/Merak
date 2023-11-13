use crate::handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use merak_database::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub async fn new() -> Self {
        let db = Db::new().await;
        db.init().await;
        AppState { db }
    }
}

pub async fn build_router() -> Router {
    let app_state = AppState::new().await;
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/tag", tag_router(app_state.clone()))
            .nest("pic", pic_router(app_state.clone()))
            .nest("/folder", folder_router(app_state.clone())),
    )
    // .with_state(app_state)
}

pub fn folder_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler::folder::get_folders))
        .route("/", post(handler::folder::add_folders))
        .route("/", put(handler::folder::upd_folder))
        .route("/", delete(handler::folder::del_folder))
        .with_state(state)
}

pub fn pic_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handler::pic::get_pics))
        .route("/", post(handler::pic::add_pics))
        .route("/", put(handler::pic::upd_pic))
        .route("/", delete(handler::pic::del_pic))
        .with_state(state)
}

pub fn tag_router(state: AppState) -> Router {
    Router::new()
        .route(
            "",
            get(handler::tag::get_tag)
                .post(handler::tag::add_tag)
                .put(handler::tag::upd_tag)
                .delete(handler::tag::del_tag),
        )
        .with_state(state)
}
