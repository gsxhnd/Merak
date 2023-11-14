use axum::Router;
use merak_database::Db;

mod folder;
mod pic;
mod tag;

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
            .nest("/tag", tag::router(app_state.clone()))
            .nest("/pic", pic::router(app_state.clone()))
            .nest("/folder", folder::router(app_state.clone())),
    )
    // .with_state(app_state)
}
