use crate::database::{self, DB};
use crate::handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::Pool<sqlx::Sqlite>,
}

pub async fn build_router() -> Router {
    let state = AppState {
        db: database::sqlite::new().await,
    };

    let app = Router::new()
        .route("/", get(handler::index))
        .with_state(state);

    app
}
