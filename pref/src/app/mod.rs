use crate::{core, peer_server};

use std::env;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use crate::discovery;
use crate::discovery::{ DiscoveryResult, DiscoveryError, NetError };
use libp2p::{Multiaddr};
use std::net::{IpAddr, Ipv4Addr};

pub struct Overlay 
{
    discov_result: DiscoveryResult,
    external_ip: IpAddr,
    server: peer_server::Server
}

impl Overlay 
{
    pub async fn new() -> Overlay 
    {
        core::pref_log::init_styled_logger();
        log::info!("Initialized log");
        
        let mut ext_addr = Ipv4Addr::from_str("127.0.0.1").expect("no");
        let mut upnp_success = false;
        let first_start = Overlay::is_first_time();
        if first_start {
            let discov_res = discovery::try_upnp_setup();


            if let Ok(ip) = discov_res {
                log::info!("UPnP enabled");

                // TODO: make option for ipv6
                ext_addr = IpAddr::V4(Ipv4Addr::from_str(&ip).expect("myexternalip.com failed...");)
                upnp_success = true;
            } 
            
            else if let Err(err) = discov_res {
                match err {
                    DiscoveryError::NetError(msg) => {
                        panic!("Internet request failed: {}", msg);
                    }
                    DiscoveryError::NATError(msg) => {
                        panic!("UPnP failed: {}", msg);
                    }
                }
            }
        }

        let self_p = peer_server::PeerInfo::load_self_peer();
        let using_bt = matches!(self_p.network_space.addr.bt, Some(_));
        let auth_pass = "test_password";

        match peer_server::gen_config(self_p.capability_type, using_bt, 4, auth_pass, None) {
            Ok(()) => {
                log::info!("Initialized reticulum config");
            } 
            Err(err) => {
                panic!("UPnP failed: {}", msg);
            }
        }

        return Overlay {
            discov_result: DiscoveryResult { upnp_enabled: upnp_success },
            external_ip: ext_addr,
            
        };
    }

    pub async fn run(&self) 
    {
        

        // auth

        // sec

        // circle conn

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

