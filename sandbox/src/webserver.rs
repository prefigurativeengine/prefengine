use axum::{response::Html, routing::get, Router, extract::State};
use std::collections::HashMap;
use log;

use crate::Sandbox;
use std::sync::Arc;


async fn get_csv(State(state): State<Arc<Sandbox>>) -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn set_csv(State(state): State<Arc<Sandbox>>) -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


pub async fn start(app: Sandbox) {
    let route: HashMap<&str, &str> = HashMap::from([
        ("root", "/"),
    ]);

    let port: &str = ":3500";

    let addr: String = "127.0.0.1".to_owned() + port;

    let shared_state = Arc::new(app);
    
    let router = Router::new()
        .route(route["root"], get(get_csv))
        .route(route["set-csv"], get(set_csv))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    log::info!("Localhost server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
