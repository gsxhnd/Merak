use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use rusqlite::{Connection, Result};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let _conn = Connection::open("./data/test.db").expect("open db error");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
