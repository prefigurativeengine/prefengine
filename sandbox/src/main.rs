
extern crate pref;
use std::future::IntoFuture;

use pref::{ app::Application };
use tokio;
use std::thread as thread;

// TODO: simplify startup procedure; no run method 
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
    webserver::start(s);
}
