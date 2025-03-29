use crate::peer_server::peer::{PeerCapability, SelfPeerInfo};
use crate::{core, peer_server};

use std::{env, vec};
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread::sleep;
use std::time;
use crate::discovery;
use crate::discovery::{ NATConfig };
use std::net::{AddrParseError, IpAddr, Ipv4Addr};
use std::process::{Child, Command, Stdio};
use peer_server::ret_util;
use peer_server::PeerStore;
use std::error::Error;

pub struct Application 
{
    nat: NATConfig,
    client: peer_server::Client,
    listener: peer_server::Listener,
    online_peers: Arc<Mutex<PeerStore>>,
    ret_process: Child
}

struct FirstStartRet {
    pub proc: Child,
    pub hash: String
}

impl Application 
{
    pub async fn new() -> Application 
    {
        core::pref_log::init_styled_logger();
        log::info!("Initialized log");
        
        let nat_conf = NATConfig::new();
        
        if nat_conf.auto_port_forward {
            log::info!("Auto port-forward enabled");
        } 
        else {
            // TODO: add some sort of message to confirm manual portforwarding
            log::info!("Auto port-forward failed, assuming manual portforward has been done");
        }


        let first_start = Application::is_first_time();

        // first init self peer values
        let capability: PeerCapability = PeerCapability::Desktop;
        let ip_fetch_r: Result<String, Box<dyn Error>> = discovery::get_public_ip();
        if let Err(_) = ip_fetch_r {
            panic!("Fetching public IP failed");
        }

        let ip_fetch = ip_fetch_r.unwrap();

        let ip_parse_r = Ipv4Addr::from_str(&ip_fetch);
        if let Err(err) = ip_parse_r {
            panic!("Parsing public IP failed: {}", err);
        }

        let ip_fetch = ip_parse_r.unwrap();
        if first_start {
            let ret_com = Command::new("python");
            // first_start_ret will use application home path
            match core::dir::get_global_data_path(true) {
                Ok(path) => {
                    match fs::create_dir_all(path) {
                        _ => {}
                        Err(e) => panic!("Failed to create app home dir")
                    } 
                }
                Err(e) => panic!("Failed to get app home dir"),
            }

            match Application::first_start_ret(&capability, ret_com) {
                Ok(ret_output) => {
                    // make self peer now that we have our destination hash from python as well
                    SelfPeerInfo::new_self_peer(capability, ip_fetch, ret_output)
                        .map_err(|e| panic!("Creating self peer failed: {}", e));
                }
                Err(e) => panic!("Failed to first start reticulum: {}", e),
            }
        }
        

        let ret_proc: Child;
        let ret_com = Command::new("python");
        match Application::start_pyret(ret_com) {
            Ok(ret_output) => {
                ret_proc = ret_output;
            }
            Err(e) => panic!("Failed to start reticulum"),
        }
        

        if let Err(err) = peer_server::db::init() {
            panic!("Starting database failed: {}", err);
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
            ret_process: ret_proc,
            online_peers: ps,
            client: client_inst,
            listener: listen_inst
        };
    }

    fn first_start_ret(capability: &PeerCapability, mut ret_com: Command) -> Result<String, String> {
        Application::gen_ret_config(capability)?;
        let hash_b = ret_com.args(["retapi.py", "first_start"])
            .output().map_err(|err| err.to_string())?
            .stdout;


        let hash = hash_b[5..hash_b.len()].to_owned();

        // if let Err(err) = ret_r {
        //     return Err(err.to_string());
        // }
    
        // let mut ret = ret_r.unwrap();
        // let l = ret.wait_with_output();
        // l.unwrap();
        Ok(String::from_utf8(hash).map_err(|err| err.to_string())?)
        // match Self::get_dest_hash(&mut ret) {
        //     Ok(dest_hash) => {
        //         Ok(FirstStartRet {
        //             proc: ret,
        //             hash: dest_hash
        //         })
        //     }
        //     Err(e) => Err(e)
        // }
    }
    
    fn start_pyret(mut ret_com: Command) -> Result<Child, String> {
        let ret_r = ret_com.args(["retapi.py"]).spawn();
        
        if let Err(err) = ret_r {
            return Err(err.to_string());
        }
    
        let ret = ret_r.unwrap();
        Ok(ret)
    }


    fn gen_ret_config(capability: &PeerCapability) -> Result<(), String> {
        // TODO: reticulum authentication
        let auth_pass = "test_password".to_owned();

        // TODO: support ipv6
        ret_util::gen_config(capability, 4, auth_pass, None)
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

    // fn get_dest_hash(proc: &mut Child) -> Result<String, String> {
    //     // TODO: make timeout
    //     loop {
    //         match proc.stdout.take() {
    //             Some(mut retout) => {
    //                 let mut buffer = String::new();
    //                 let res_r = retout.read_to_string(&mut buffer);

    //                 if let Err(err) = res_r {
    //                     return Err(err.to_string());
    //                 }
    //                 let res = res_r.unwrap();

    //                 if buffer.starts_with("hash:") {
                        
    //                     return Ok(hash)
    //                 }
    //             }, 
    //             None => {
    //                 sleep(time::Duration::from_millis(500));
    //             }
    //         }
    //     }
    // }

    

    pub fn get_db_data(&self) -> Result<String, String> {
        return peer_server::db::db_to_str();
    }

    pub fn set_db_data(&self, new_data: String) -> Result<(), String> {
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

