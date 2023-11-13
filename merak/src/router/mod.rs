use crate::handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
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
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(handler::index))
                .route("/test", get(handler::index).post(handler::add_tag)),
        )
        .with_state(AppState::new().await)
}
