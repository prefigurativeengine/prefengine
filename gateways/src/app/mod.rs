use crate::local_server;
use crate::ssb::SSBTcpServer;
use crate::cat_log;
use crate::ssb::ssb_id;

pub struct Engine {
    ssb_server: SSBTcpServer
}

impl Engine {
    pub async fn new() -> Engine {
        let ssb_tcp_result = SSBTcpServer::new().await;
        if ssb_tcp_result.is_err() {
            panic!("Failed to init SSBTcpServer.");
        }

        let ssb_tcp = ssb_tcp_result.unwrap();

        return Engine {
            ssb_server: ssb_tcp
        };
    }

    pub fn run(&self) {
        cat_log::init_styled_logger();
        log::info!("Initialized log");

        if (first_start) {
            ssb_id::first_time_id_gen();
        }

        // auth

        // sec

        // circle conn

        crate::local_server::start();
    }

    
}


