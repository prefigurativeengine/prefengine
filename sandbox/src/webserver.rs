use axum::{extract::{State, Extension}, response::Html, routing::{get, post}, Router};
use serde_json;
use std::{collections::HashMap, sync::Mutex};
use log;

use crate::Sandbox;
use std::sync::Arc;


async fn get_csv(Extension(app): Extension<Arc<Mutex<Sandbox>>>) -> Html<String> {
    let app_locked = app.lock().unwrap();
    let csv_str_r = app_locked.eng.get_db_data();
    if let Err(err) = csv_str_r {
        log::error!("Getting csv string failed: {}", err);
        return Html("<h1>Err</h1>".to_owned());
    }

    Html(csv_str_r.unwrap())
}

async fn set_csv(
    Extension(app): Extension<Arc<Mutex<Sandbox>>>, 
    payload: String
    ) -> Html<&'static str> {
    let mut app_locked = app.lock().unwrap();
    if let Err(err) = app_locked.eng.update_db(payload) {
        log::error!("Setting csv string failed: {}", err);
        return Html("<h1>Err!</h1>");
    }
    Html("<h1>Success</h1>")
}


pub async fn start(app: Sandbox) {
    let route: HashMap<&str, &str> = HashMap::from([
        ("root", "/"),
        ("set_csv", "/set-csv"),
    ]);

    let port: &str = ":3500";

    let addr: String = "127.0.0.1".to_owned() + port;

    let shared_app: Arc<Mutex<Sandbox>> = Arc::new(
        Mutex::new(
            app
        )
    );
    
    let router = Router::new()
        .route(route["root"], get(get_csv))
        .route(route["set_csv"], post(set_csv))
        .layer(Extension(shared_app));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    log::info!("Localhost server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
