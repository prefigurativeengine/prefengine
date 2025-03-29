use axum::{extract::{State, Json}, response::Html, routing::{get, post}, Router};
use serde_json;
use std::collections::HashMap;
use log;

use crate::Sandbox;
use std::sync::Arc;


async fn get_csv(State(state): State<Arc<Sandbox>>) -> Html<String> {
    let csv_str_r = state.eng.get_db_data();
    if let Err(err) = csv_str_r {
        log::error!("Getting csv string failed: {}", err);
        return Html("<h1>Err!</h1>".to_owned());
    }

    Html(csv_str_r.unwrap())
}

async fn set_csv(
    State(state): State<Arc<Sandbox>>, 
    payload: String
    ) -> Html<&'static str> {
    //if let serde_json::Value::String(inner_pl) = payload {
    if let Err(err) = state.eng.set_db_data(payload) {
        log::error!("Setting csv string failed: {}", err);
        return Html("<h1>Err!</h1>");
    }
    Html("<h1>Hello, World!</h1>")
    //}
}


pub async fn start(app: Sandbox) {
    let route: HashMap<&str, &str> = HashMap::from([
        ("root", "/"),
        ("set_csv", "/set-csv"),
    ]);

    let port: &str = ":3500";

    let addr: String = "127.0.0.1".to_owned() + port;

    let shared_state = Arc::new(app);
    
    let router = Router::new()
        .route(route["root"], get(get_csv))
        .route(route["set_csv"], post(set_csv))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    log::info!("Localhost server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
