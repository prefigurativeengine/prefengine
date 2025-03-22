use crate::{core, peer_server};

use std::env;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread::sleep;
use std::time;
use crate::discovery;
use crate::discovery::{ NATConfig };
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Command, Child};
use peer_server::ret_util;
use peer_server::PeerStore;

// instead of discov_result & external_ip, use nat part of model

pub struct Application 
{
    nat: NATConfig,
    client: peer_server::Client,
    listener: peer_server::Listener,
    online_peers: Arc<Mutex<PeerStore>>,
    ret_process: Child
}

impl Application 
{
    pub async fn new() -> Application 
    {
        core::pref_log::init_styled_logger();
        log::info!("Initialized log");
        
        // TODO: make this not mut
        let mut ext_addr = Ipv4Addr::from_str("127.0.0.1").expect("no");
        let mut upnp_success = false;
        let nat_conf = NATConfig::new();
        
        if nat_conf.auto_port_forward {
            log::info!("Auto port-forward enabled");
        } 
        else {
            panic!("Auto port-forward failed, aborting");
        }
 
        let self_p_r = peer_server::peer::SelfPeerInfo::load_self_peer();
        if let Err(err) = self_p_r {
            panic!("Getting self peer failed: {}", err);
        }
        let self_p = self_p_r.unwrap();

        let using_bt = matches!(self_p.addr.bt, Some(_));

        // TODO: reticulum authentication
        let auth_pass = "test_password".to_owned();

        match ret_util::gen_config(self_p.cap_type, using_bt, 4, auth_pass, None) {
            Ok(()) => {
                log::info!("Initialized reticulum config");
            },
            Err(err) => {
                panic!("Reticulum configuration failed: {}", err);
            }
        }

        let first_start = Application::is_first_time();

        let ret_args = {
            if first_start {
                vec!["retapi.py", "first_start"]
            } else {
                vec!["retapi.py"]
            }
        };

        let mut ret = Command::new("python")
            .args(ret_args)
            .spawn()
            .expect("failed to execute retapi.py");

        // TODO: make timeout
        loop {
            match ret.stdout.take() {
                Some(mut retout) => {
                    let mut buffer = String::new();
                    let res = retout.read_to_string(&mut buffer)
                        .expect("failed to read first stdout from retapi.py");

                    if buffer.starts_with("Client listening") {
                        log::info!("Recieved Reticulum API listening message");
                        break;
                    }
                },
                None => {
                    sleep(time::Duration::from_millis(500));
                }
            }
        }

        
        let ps: Arc<Mutex<PeerStore>> = Arc::new(
            Mutex::new(
                PeerStore::new()
            )
        );

        let listen_inst = peer_server::Listener::new(&ps);
        let client_inst = peer_server::Client::new(&ps);

        return Application {
            nat: nat_conf,
            ret_process: ret,
            online_peers: ps,
            client: client_inst,
            listener: listen_inst
        };
    }

    pub fn stop() {
        let discov_res = discovery::rmv_upnp_setup();

        match discov_res {
            Ok(()) => {
                log::info!("UPnP port disabled");
            }
            Err(msg) => {
                log::error!("Failed to disable UPnP port: {}", msg);
            }
        }
    }


    pub fn get_db_data() -> Result<String, String> {
        return peer_server::db::db_to_str();
    }

    pub fn set_db_data(new_data: String) -> Result<(), String> {
        return peer_server::db::append_chg(&new_data);
    }

    pub fn update_db(&mut self, rows: String) -> Result<(), String> {
        return peer_server::db::process_local_change(rows, &mut self.client);
    }

    fn is_first_time() -> bool 
    {
        // TODO: improve this, maybe change to looking for config file
        let mut path = env::current_dir()
            .expect("Unable to read current working directory");
        path.push("DO_NOT_DELETE_OR_MOVE");

        if (Path::new(&path).exists()) {
            return false
        } else {
            fs::File::create(path).expect("Unable to write 'first start file' to current working directory");
            return true;
        }
    }

     
}

