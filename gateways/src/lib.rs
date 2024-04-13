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

// use externs
use log;

pub trait Server {
    fn run(&self);
}

pub fn main(s: Box<dyn Server>) {
    s.run();
}
