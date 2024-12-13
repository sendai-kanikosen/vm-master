use axum::{http::StatusCode, routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/healthz", get(healthcheck));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
