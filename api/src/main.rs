extern crate prefengine;

use prefengine::{ app::Application };
use tokio;

// pub struct App {
//     eng: Application
// }

// impl Sandbox {
//     async fn new() -> Sandbox {
//         let e: Application = Application::new().await;
//         return Sandbox { eng: e };
//     }
// }

mod webserver;

#[tokio::main]
async fn main() {
    let a: Application = Application::new().await;
    webserver::start(a).await;
}
