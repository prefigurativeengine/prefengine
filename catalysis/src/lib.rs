#[cfg(not(target_os = "windows"))]
compile_error!("Windows only");

// export app for sandbox
pub mod app;
use app::cat_engine;

pub struct Server {
    eng: cat_engine::Engine
}

impl Server {
    fn new() -> Server {
        let e: cat_engine::Engine = cat_engine::Engine::new();
        return Server { eng: e };
    }

    fn run(&self) {
        self.eng.run();
    }
}

pub fn main () {
    let s: Box<Server> = Box::new(Server::new());
    s.run();
}






