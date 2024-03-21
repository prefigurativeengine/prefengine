#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export for sandbox
pub mod app;
pub mod cat_log;

// export for app
mod local_server;

// private mod

// use externs
use log;


pub trait Server {
    fn run(&self);
}

pub fn main(s: Box<dyn Server>) {
    cat_log::init_styled_logger();
    log::warn!("Initialized log");

    s.run();
}
