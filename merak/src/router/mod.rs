use crate::handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};

use crate::database::{self};

#[derive(Clone)]
pub struct AppState {
    pub db: database::db,
}

impl AppState {
    pub async fn new() -> Self {
        let db = database::db::new().await;
        AppState { db }
    }
}

pub struct AppStates<DB> {
    pub db: DB,
}

pub async fn build_router() -> Router {
    // `GET /` goes to `root`
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(handler::index))
                .route("/test", get(handler::index)),
        )
        .with_state(AppState::new().await);

    app
}
