//pub use self::cat_engine;
use crate::local_server;

pub mod cat_engine {

    pub struct Engine {
        
    }
    
    impl Engine {
        pub fn new() -> Engine {
            return Engine {};
        }

        pub fn run(&self) {
            crate::local_server::start();
        }

        
    }
}

