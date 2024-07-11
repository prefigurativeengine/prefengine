
extern crate gateways;
use std::future::IntoFuture;

use gateways::{ app::Engine as cat_engine, Server};
use tokio;

pub struct Sandbox {
    eng: cat_engine
}

impl Sandbox {
    async fn new() -> Sandbox {
        let e: cat_engine = cat_engine::new().await;
        return Sandbox { eng: e };
    }
}

impl Server for Sandbox  {
    async fn run(&self) {
        self.eng.run().await;
    }
}

#[tokio::main]
async fn main() {
    let s: Box<Sandbox> = Box::new(Sandbox::new().await);
    gateways::main(s);
}
