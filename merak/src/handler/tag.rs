use crate::router::AppState;
use axum::extract::State;

use merak_database::tag;

pub async fn add_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = tag::add_tag(&state.db.conn).await;
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
    // tag::del_tag(1).await;
    let _ = &state.db.get_all_tags().await;
    print!("wait db");
    "Save tag"
}

pub async fn upd_tag(state: State<AppState>) -> &'static str {
    print!("wait db");
    let _ = &state.db.add_tag().await;
    print!("wait db");
    "Save tag"
}
