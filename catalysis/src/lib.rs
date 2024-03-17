#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export app for sandbox
pub mod app;
pub mod cat_log;

// private mod


// use externs
use log;


pub trait Server {
    fn run(&self);
}

pub fn main(s: Box<dyn Server>) {
    cat_log::init_styled_logger();
    log::warn!("Initialized log");
    let var = 5;
    log::info!("Hello var: {var}");
    s.run();
}
