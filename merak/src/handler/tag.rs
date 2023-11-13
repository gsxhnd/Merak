use crate::router::AppState;
use axum::extract::State;

pub async fn add_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}

pub async fn del_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}

pub async fn get_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}

pub async fn upd_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}
