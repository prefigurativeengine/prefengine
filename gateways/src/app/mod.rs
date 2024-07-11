use crate::{local_server, ssb};
use crate::ssb::SSBTcpServer;
use crate::cat_log;
use crate::ssb::ssb_id;

use std::env;
use std::path::Path;
use std::fs;

pub struct Engine 
{
    ssb_server: SSBTcpServer
}

impl Engine 
{
    pub async fn new() -> Engine 
    {
        cat_log::init_styled_logger();
        log::info!("Initialized log");
        
        let ssb_tcp_result = SSBTcpServer::new().await;
        if ssb_tcp_result.is_err() {
            panic!("{}", ssb_tcp_result.err().unwrap());
        }

        let ssb_tcp = ssb_tcp_result.unwrap();

        return Engine {
            ssb_server: ssb_tcp
        };
    }

    pub async fn run(&self) 
    {
        let first_start = Engine::is_first_time();
        if (first_start) {
            ssb_id::first_time_id_gen().await;
        }

        // auth

        // sec

        // circle conn

        crate::local_server::start().await;
    }

    fn is_first_time() -> bool 
    {
        // TODO: make this a global constant
        let mut path = env::current_dir()
            .expect("Unable to read current working directory");
        path.push("DO_NOT_DELETE_OR_MOVE");

        // TODO: for windows, maybe replace with registry value lookup
        if (Path::new(&path).exists()) {
            return false
        } else {
            fs::File::create(path).expect("Unable to write 'first start file' to current working directory");
            return true;
        }
    }

    
}


