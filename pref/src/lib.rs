#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export for sandbox
pub mod app;
pub mod core;

// export for app


// private mods


// use externs
mod discovery;
mod peer_server;


