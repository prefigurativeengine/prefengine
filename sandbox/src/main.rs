
extern crate catalysis;
use catalysis::{ app::cat_engine as cat_engine, Server };


pub struct Sandbox {
    eng: cat_engine::Engine
}

impl Sandbox {
    fn new() -> Sandbox {
        let e: cat_engine::Engine = cat_engine::Engine::new();
        return Sandbox { eng: e };
    }
}

impl Server for Sandbox  {
    fn run(&self) {
        self.eng.run();
    }
}


fn main() {
    let s: Box<Sandbox> = Box::new(Sandbox::new());
    catalysis::main(s);
}
