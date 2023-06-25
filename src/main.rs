use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/", get(|| async { Html("<h1>Bloom</h1>") }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
