mod app;
mod database;
mod handler;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let r = app::build_router().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(r.into_make_service())
        .await
        .unwrap();
}
