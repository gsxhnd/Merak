use crate::router::AppState;
use axum::extract::State;
// use axum::Json;

pub async fn index(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.get_all_tags().await;
    print!("wait db");
    "Get All tags"
}

pub async fn add_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}
