extern crate prefengine;

use prefengine::app::Application;
use tokio;

mod webserver;

#[tokio::main]
async fn main() {
    let a: Application = Application::new().await;
    webserver::start(a).await;
}
