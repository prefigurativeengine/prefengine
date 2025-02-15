#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export for sandbox
pub mod app;
pub mod core;

// export for app


// private mods


// use externs
mod discovery;

pub trait Server {
    async fn run(&self);
}

// TODO: maybe avoid generics
pub async fn main<T>(s: Box<T>) where T: Server {
    s.run().await;
}
