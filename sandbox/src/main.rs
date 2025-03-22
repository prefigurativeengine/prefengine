extern crate pref;

use pref::{ app::Application };
use tokio;

pub struct Sandbox {
    eng: Application
}

impl Sandbox {
    async fn new() -> Sandbox {
        let e: Application = Application::new().await;
        return Sandbox { eng: e };
    }
}

mod webserver;

#[tokio::main]
async fn main() {
    let s: Sandbox = Sandbox::new().await;
    webserver::start(s).await;
}
