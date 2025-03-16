
extern crate pref;
use std::future::IntoFuture;

use pref::{ app::Application as pref_engine, Server};
use tokio;
use std::thread as thread;

// TODO: simplify startup procedure; no run method 
pub struct Sandbox {
    eng: pref_engine
}

impl Sandbox {
    async fn new() -> Sandbox {
        let e: pref_engine = pref_engine::new().await;
        return Sandbox { eng: e };
    }
}


impl Server for Sandbox  {
    async fn run(&self) {
        self.eng.run().await;
    }
}


mod webserver;



#[tokio::main]
async fn main() {
    thread::spawn(|| webserver::start);
    let s: Box<Sandbox> = Box::new(Sandbox::new().await);
    pref::main(s).await;
}
