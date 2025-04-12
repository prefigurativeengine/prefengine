use axum::{
    Router,
    extract::{Extension, State},
    response::Html,
    routing::{get, post},
};
use log;
use prefengine::app::Application;
use serde_json;
use std::{collections::HashMap, sync::Mutex};

use std::sync::Arc;

async fn get_ret_dest(Extension(app): Extension<Arc<Mutex<Application>>>) -> Html<String> {
    let app_locked = app.lock().unwrap();
    let selfp = app_locked.get_self_peer();
    if let Err(err) = selfp {
        log::error!("Getting csv string failed: {}", err);
        return Html("<h1>Err</h1>".to_owned());
    }

    Html(selfp.unwrap().addr.dest_hash)
}

async fn get_csv(Extension(app): Extension<Arc<Mutex<Application>>>) -> Html<String> {
    let app_locked = app.lock().unwrap();
    let csv_str_r = app_locked.get_db_data();
    if let Err(err) = csv_str_r {
        log::error!("Getting csv string failed: {}", err);
        return Html("<h1>Err</h1>".to_owned());
    }

    Html(csv_str_r.unwrap())
}

async fn set_csv(
    Extension(app): Extension<Arc<Mutex<Application>>>,
    payload: String,
) -> Html<&'static str> {
    let mut app_locked = app.lock().unwrap();
    if let Err(err) = app_locked.update_db(payload) {
        log::error!("Setting csv string failed: {}", err);
        return Html("<h1>Err!</h1>");
    }
    Html("<h1>Success</h1>")
}

async fn add_temp(
    Extension(app): Extension<Arc<Mutex<Application>>>,
    payload: String,
) -> Html<&'static str> {
    let app_locked = app.lock().unwrap();
    if let Err(err) = app_locked.add_temp_peer(payload) {
        log::error!("Adding a temp peer failed: {}", err);
        return Html("<h1>Err!</h1>");
    }
    Html("<h1>Success</h1>")
}

async fn transform_temp_peer(
    Extension(app): Extension<Arc<Mutex<Application>>>,
) -> Html<&'static str> {
    let mut app_locked = app.lock().unwrap();
    if let Err(err) = app_locked.all_temp_peers_to_peer() {
        log::error!("Transforming temp peers failed: {}", err);
        return Html("<h1>Err!</h1>");
    }
    Html("<h1>Success</h1>")
}

pub async fn start(app: Application) {
    let route: HashMap<&str, &str> = HashMap::from([
        ("get_csv", "/"),
        ("set_csv", "/set-csv"),
        ("transform_temp_peer", "/transform-temp"),
        ("add_temp", "/add-temp"),
        ("get_ret_dest", "/get-dest"),
    ]);

    let port: &str = ":3500";

    let addr: String = "127.0.0.1".to_owned() + port;

    let shared_app: Arc<Mutex<Application>> = Arc::new(Mutex::new(app));

    let router = Router::new()
        .route(route["get_csv"], get(get_csv))
        .route(route["set_csv"], post(set_csv))
        .route(route["transform_temp_peer"], post(transform_temp_peer))
        .route(route["add_temp"], post(add_temp))
        .route(route["get_ret_dest"], get(get_ret_dest))
        .layer(Extension(shared_app));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    log::info!(
        "Localhost server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, router).await.unwrap();
}
