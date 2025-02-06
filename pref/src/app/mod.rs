use crate::core;

use std::env;
use std::path::Path;
use std::fs;

pub struct Engine 
{
    
}

impl Engine 
{
    pub async fn new() -> Engine 
    {
        core::pref_log::init_styled_logger();
        log::info!("Initialized log");
        
        // let ssb_tcp_result = SSBTcpServer::new().await;
        // if ssb_tcp_result.is_err() {
        //     panic!("{}", ssb_tcp_result.err().unwrap());
        // }

        // let ssb_tcp = ssb_tcp_result.unwrap();

        return Engine {
        };
    }

    pub async fn run(&self) 
    {
        let first_start = Engine::is_first_time();
        if first_start {
            
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

        // TODO: for windows, maybe replace with registry value lookup, or json value in config file
        if (Path::new(&path).exists()) {
            return false
        } else {
            fs::File::create(path).expect("Unable to write 'first start file' to current working directory");
            return true;
        }
    }

     
}


