use crate::local_server;

pub mod cat_engine {

    pub struct Engine {
        
    }
    
    impl Engine {
        pub fn new() -> Engine {
            return Engine {};
        }

        pub fn run(&self) {
            cat_log::init_styled_logger();
            log::info!("Initialized log");

            if (first_start) {
                ssb::ssb_id::first_time_id_gen();
            }

            // auth

            // sec

            // circle conn

            crate::local_server::start();
        }

        
    }
}

