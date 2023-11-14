use crate::router::AppState;
use axum::extract::State;

pub async fn add_pics(state: State<AppState>) -> &'static str {
    // print!("wait db");
    // let _ = &state.db.add_tag().await;
    // print!("wait db");
    "Save tag"
}

pub async fn del_pic(state: State<AppState>) -> &'static str {
    // print!("wait db");
    // let _ = &state.db.add_tag().await;
    // print!("wait db");
    "Save tag"
}

pub async fn get_pics(state: State<AppState>) -> &'static str {
    // print!("wait db");
    // let _ = &state.db.add_tag().await;
    // print!("wait db");
    "Save tag"
}

pub async fn upd_pic(state: State<AppState>) -> &'static str {
    // print!("wait db");
    // let _ = &state.db.add_tag().await;
    // print!("wait db");
    "Save tag"
}
