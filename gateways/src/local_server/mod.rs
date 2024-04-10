use axum::{response::Html, routing::get, Router};
use std::collections::HashMap;
use log;

#[tokio::main]
pub async fn start() {
    let route: HashMap<&str, &str> = HashMap::from([
        ("root", "/"),
    ]);

    let port: &str = ":3500";

    let addr: String = "127.0.0.1".to_owned() + port;
    
    let router = Router::new().route(route["root"], get(handler));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_graceful_shutdown(shutdown_signal())
        .unwrap();

    log::info!("local_server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
