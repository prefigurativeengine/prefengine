#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export for sandbox
pub mod app;
pub mod cat_log;

// export for app
mod local_server;

// private mod
mod ssb;

use std::fs::File;
use tokio::macros::support::Future;

// use externs
use log;

pub trait Server {
    async fn run(&self);
}

// TODO: maybe avoid generics
pub async fn main<T>(s: Box<T>) where T: Server {
    s.run().await;
}
