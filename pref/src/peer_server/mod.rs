use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use std::{fs, thread}; 

mod peer;
use peer::Peer;
use serde::Serialize;

use crate::peer_server::peer::{self, *};

mod connection;
use crate::peer_server::connection as conn;

mod db;
use crate::peer_server::db as peer_db;

use crate::core::{self, *};
use configparser::ini::Ini;

const RET_URL: &str = "127.0.0.1:3502";
const PREF_URL: &str = "127.0.0.1:3501";

pub fn gen_config(
    c_type: peer::PeerCapability, 
    bt: bool, 
    log_level: u32, 
    auth_pass: String, 
    ipv6_addr: Option<Ipv6Addr>) -> Result<(), String> {
    // Get needed info; c type, available socket interfaces. assumes a tcp interface is connected to internet.

    let mut config = Ini::new();
    config.load("reticulum_dummy_config.conf")?;
    
    match c_type {
        peer::PeerCapability::Desktop => {
            config.set("reticulum", "enable_transport", Some("No"));
            config.set("reticulum", "share_instance", Some("No"));
        }

        peer::PeerCapability::Server | peer::PeerCapability::PtpRelay => {
            config.set("reticulum", "enable_transport", Some("Yes"));
        }
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    config.set("logging", "loglevel", Some(log_level));

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be dynamic and not just based 
    // off a file
    config.set("TCP Server Interface", "passphrase", Some(auth_pass));

    match ipv6_addr {
        Some(addr) => {
            config.set("TCP Server Interface", "listen_ip", Some(addr));
            config.set("TCP Client Interface", "target_host", Some(addr));    
        }
    }

    return config.write("reticulum_config.conf");
}


pub struct Server {
    ret_api_listener: net::TcpListener,
    ret_api_conn: net::TcpStream,
    peers: Vec<Peer>,
}

const FO_RECONNECT_ACTION: &str = "fo_reconnect";
const SEND_ACTION: &str = "send";

impl Server {
    pub fn new(url: String) -> Server {
        let server = Server {
            ret_api_listener: TcpListener::bind(PREF_URL)
                .expect("Could not start the reticulum listener"),
            ret_api_conn: TcpStream::connect(RET_URL)
                .expect("Could not connect to reticulum");
            peers: Vec::new(),
        };

        server
    }

    pub fn start(&self) {
        self.peer_connect_all();
        thread::spawn(self.ret_listen);
    }

    fn peer_connect_all(&self) -> Result<(), String> {
        if let Ok(peers) = PeerInfo::load_remote_peers() {
            for peer in peers {
                match self.peer_connect(peer) {
                    Ok(()) => {
                        log::info!("Sent reconnect msg to reverse proxy for {}", peer.id.parent_id);
                    },
        
                    Err(error_s) => {
                        log::error!("Failed to send reconnect msg: {}", error_s);
                    }
                }
            }
            return Ok(());
        }
        else {
            return Err("Failed to load peers".to_owned())
        }
    }

    fn peer_connect(&self, peer: PeerInfo) -> Result<usize, std::io::Error> {
        if matches!(peer.p_type, PeerType::Local { local_space: _ }) {
            return Err(("Local peer cannot be connected to."))
        }
        
        // TODO: run through a list of connection tactics according to values in peerinfo    
        let json_s = format_for_ret(peer.id.parent_id, FO_RECONNECT_ACTION, None);
        let res = self.ret_send(json_s);
        return res;
    }

    fn format_for_ret(id: String, action: String, data: Option<HashMap<String, String>>) -> String {
        let hm_dto: HashMap<String, String> = HashMap::new();

        hm_dto.insert("id".to_owned(), id.clone());
        hm_dto.insert("action".to_owned(), action.clone());
        
        if matches!(data, Some(_)) {
            hm_dto.extend(data.unwrap());
        }

        return serde_json::to_string(hm_dto);
    }

    fn ret_send(&self, data: String) -> Result<usize, std::io::Error> {
        return self.ret_api_conn.write(data.as_bytes());
    }

    use serde_json::{Result, Value};
    fn ret_listen(&self) {
        for stream in self.ret_api_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let buf = String::new();

                    match stream.read_to_string(buf) {
                        Ok(usize) => {
                            let serde_res: Result<HashMap<String, Value>, Error> = serde_json::from_str(s);
                            if matches!(serde_res, Some(_)) {
                                self.dispatch_ret_resp(serde_res.unwrap());
                            } else {
                                log::error!("Failed to decode ret proxy message into json: {}", serde_res.unwrap());
                            }
                        }

                        Err(err) => {
                            log::error!("Failed to decode ret proxy message: {}", err);
                        }
                    }
                },
                Err(error_s) => log::error!("Error when tried to use stream: {}", error_s),
            }
        }
    }

    fn dispatch_ret_resp(resp: HashMap<String, Value>) {
        match resp.get("value") {
            "new_peer" => {
                if self.check_peer_req(resp) {
                    self.add_peer(resp);
                }
            }
            "res_fin" => {
                self.recieved_db_change(resp);
            }
        }
    }

    fn check_peer_req(&self, resp: HashMap<String, Value>) -> bool {
        for peer in self.get_disconnected_peers() {
            if peer.info.id.child_dest_id == resp.get("id") {
                return true;
            }
        }
        return false;
    }

    // assumes resp is a known peer
    fn add_peer(&self, new_peer: HashMap<String, Value>) -> Result<(), String> {
        // add to runtime list
        let new_p: Peer;
        
        let disconn_peers_res: Result<Vec<PeerInfo>, String> = self.get_disconn_peers();
        if disconn_peers_res.is_err() {
            return Err("Getting disconnected peers failed");
        }

        for p_info in disconn_peers_res.unwrap() {
            if p_info.id.child_dest_id == new_peer.get("id") {
                new_p = Peer::new(p_info);
                self.peers.push(p);
            }
        }
        
        // add to persistant peers if new
        PeerInfo::append_peers_to_disk(vec![new_p]);
    }

    fn get_disconn_peers(&self) -> Result<Vec<PeerInfo>, String> {
        let disk_peer_res: Result<Vec<PeerInfo>, String> = PeerInfo::load_remote_peers();
        match disk_peer_res {
            Ok(d_peers) => {
                Ok(self.filter_pinfo_for_disconn(d_peers));
            }
            Err(msg) => {
                log::error!(msg);
                Err(("Failed to get remote peers from disk"));
            }
        }
    }

    fn filter_pinfo_for_disconnected(self, p_infos: &Vec<PeerInfo>) -> Vec<PeerInfo> {
        let mut disconn_peers = vec![];

        for peer_info in p_infos {
            // brute forcing, but peers in a given overlay have upper limit of 150

            let current_id = peer_info.info.id.parent_id;
            let found: bool;

            for online_peer in self.peers {
                if current_id == online_peer.id.parent_id {
                    found = true;
                }
            }

            if !found {
                disconn_peers.push(value);
            }
        }
        
        return disconn_peers;
    }

    fn recieved_db_change(resp: HashMap<String, Value>) {

    }

    fn send_db_change(resp: HashMap<String, Value>) {

    }

    
    fn on_new_peer_connect(&self, stream: TcpStream) {
        // check if a valid ip addr
        if self.valid_peer_addrs.contains(stream.peer_addr()) {
            // add as connection
        } else {
            // if not a valid ip, peek msg to see full of identity of remote 
            stream.read_timeout();
            // if invalid identity, drop

            // if valid, add as connection and update ip addr
        }
    }

    fn try_traversal_methods() {

    }

    fn handle_conn_failure() {

    }

}
