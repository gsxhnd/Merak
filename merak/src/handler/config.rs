use crate::router::AppState;
use axum::extract::State;
// use axum::Json;

pub async fn index(state: State<AppState>) -> &'static str {
    // sqlx::query("insert into person (name) values (1)")
    //     .execute(&db)
    //     .await
    //     .unwrap();
    let _ = &state.db.ping();
    "Index"
}
