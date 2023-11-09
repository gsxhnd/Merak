use crate::app::AppState;
use crate::database::DB;
use axum::extract::State;
use axum::Json;
use sqlx::{Database, Sqlite};

pub async fn index(
    // State(AppState { db }): State<AppState<D>>,
    State(AppState { db }): State<AppState>,
) -> &'static str {
    // let items =

    // let view = IndexViewModel {
    //     title: "All Items",
    //     items,
    // };

    sqlx::query("insert into person (name) values (1)")
        .execute(&db)
        .await
        .unwrap();

    // Html(templates.render("index", &view).unwrap())
    "Index"
}
