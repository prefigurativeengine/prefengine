#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export app for sandbox
pub mod app;

pub trait Server {
    fn run(&self);
}

pub fn main(s: Box<dyn Server>) {
    s.run();
}
