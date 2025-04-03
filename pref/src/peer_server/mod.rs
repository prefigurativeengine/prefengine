use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use std::{fs, thread}; 

pub mod peer;
use peer::{RemotePeer, TempPeerInfo};
use peer::RemotePeerInfo;
use peer::PeerCapability;
use serde_json::Value;
use std::sync::Arc;
mod connection;

pub mod db;
use db as peer_db;

pub mod ret_util;

use serde_json::{Result as s_Result};
use serde_json::Error as s_Error;
use std::sync::Mutex;

const RET_URL: &str = "127.0.0.1:3502";
const PREF_LISTEN_URL: &str = "127.0.0.1:3501";


pub struct Client {
    ret_api_conn: net::TcpStream,
    peers: Arc<Mutex<PeerStore>>,
}

const FO_RECONNECT_ACTION: &str = "fo_reconnect";
const SEND_ACTION: &str = "send";


impl Client {
    pub fn new(ps: &Arc<Mutex<PeerStore>>) -> Result<Client, String> {
        let client_res = {
            let mut client_new = Client {
                ret_api_conn: TcpStream::connect(RET_URL)
                    .map_err(|e| e.to_string())?,
                peers: Arc::clone(ps),
            };

            if peer::RemotePeerInfo::load_remote_peers()?.is_empty() && 
                peer::TempPeerInfo::load_expected_temps()?.is_empty() {
                return Err("No peers to connect to.".to_owned());
            }

            Ok(client_new)
        };

        client_res
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.peer_connect_all()
    } 

    pub fn send_db_change(&mut self, change: String) -> Result<(), String> {
        let mut change_map = HashMap::new();
        change_map.insert("action".to_owned(), "send".to_owned());
        change_map.insert("change".to_owned(), change);

        let json_s = Client::format_for_ret(None, SEND_ACTION, Some(change_map))
            .map_err(|err| err.to_string())?;

        match self.ret_send(json_s) {
            Ok(size) => Ok(()),
            Err(err) => return Err(err.to_string())
        }
    }

    pub fn peer_connect_all(&mut self) -> Result<(), String> {
        if let Ok(peers) = RemotePeerInfo::load_remote_peers() {
            for peer in peers {
                match self.peer_connect(&peer.addr.dest_hash) {
                    Ok((_)) => {
                        log::info!("Sent reconnect msg to reverse proxy for {}", peer.id.value);
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

    fn peer_connect(&mut self, peer_dest: &String) -> Result<usize, std::io::Error> {
        // TODO: run through a list of connection tactics according to values in peerinfo 
        // TODO: check if connect attempt succeeded

        let id_cpy = peer_dest.clone();    
        let json_s_r = Client::format_for_ret(Some(id_cpy), FO_RECONNECT_ACTION, None);
        if let Err(err) = json_s_r {
            let err_obj = std::io::Error::new(std::io::ErrorKind::Other, err);
            return Err(err_obj);
        }

        let json_s = json_s_r.unwrap();
        let res = self.ret_send(json_s);
        return res;
    }

    fn format_for_ret(id: Option<String>, action: &str, data: Option<HashMap<String, String>>) -> Result<String, String> {
        let mut hm_dto: HashMap<String, String> = HashMap::new();

        if let Some(id_val) = id {
            hm_dto.insert("id".to_owned(), id_val);
        }
        
        hm_dto.insert("action".to_owned(), action.to_owned());
        
        if matches!(data, Some(_)) {
            hm_dto.extend(data.unwrap());
        }
 
        match serde_json::to_string(&hm_dto) {
            Ok(str_dto) => Ok(str_dto),
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    fn ret_send(&mut self, data: String) -> Result<usize, std::io::Error> {
        return self.ret_api_conn.write(data.as_bytes());
    }

    fn try_traversal_methods() {

    }

    fn handle_conn_failure() {

    }

}


pub struct Listener {
    pub inner_listener: net::TcpListener,
    peers: Arc<Mutex<PeerStore>>
}

impl Listener {
    pub fn new(ps: &Arc<Mutex<PeerStore>>) -> Listener {
        let listen = Listener {
            inner_listener: TcpListener::bind(PREF_LISTEN_URL)
                .expect("Could not start the reticulum listener"),
            peers: Arc::clone(ps),
        };

        listen
    }

    // must be called on a seperate thread
    pub fn start(self) {
        for stream in self.inner_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = String::new();

                    match stream.read_to_string(&mut buf) {
                        Ok(usize) => {
                            self.dispatch_ret_resp(buf);
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
    
    fn dispatch_ret_resp(&self, resp: String) -> Result<(), String> {
        // HACK: use rpc library
        let new_peer = "{\"action\":\"new_peer";
        let resc_fin = "{\"action\":\"resc_fin";

        match &resp[0..19] {
            new_peer => {
                let resp_map = serde_json::from_str(&resp)
                    .map_err(|err| err.to_string())?;

                let mut ps = self.peers.lock().unwrap();
                
                let check_res = ps.check_peer_req(&resp_map);
                if check_res.is_ok() {
                    ps.add_peer(resp_map);
                    return Ok(());
                } 
                else {
                    return Err("Peer validation failed".to_owned());
                }
            },
            resc_fin => {
                return peer_db::process_remote_change(resp);
            }
        }
    }
}

// for managing in-memory storage of online peers 
pub struct PeerStore {
    peers: Vec<RemotePeer>,
}

impl PeerStore {
    pub fn new() -> PeerStore {
        let ps = PeerStore {
            peers: vec![],
        };

        ps
    }

    fn check_peer_req(&self, resp: &HashMap<String, Value>) -> Result<bool, String> {
        let dis_peers = self.get_disconn_peers();
        if dis_peers.is_err() {
            return Err("Getting disconnected peers failed".to_owned());
        }

        for peer in dis_peers.unwrap() {
            if let Some(id_val) = resp.get("id") {
                if let Value::String(id_s) = id_val {
                    if peer.addr.dest_hash == *id_s {
                        return Ok(true);
                    }
                } 
                
                else {
                    return Err("incorrect value for id".to_owned()) 
                }
            }
        }
        return Ok(false);
    }
    
    fn add_peer(&mut self, new_peer: HashMap<String, Value>) -> Result<(), String> {
        // add to runtime list
        let mut new_p: RemotePeer;
        
        let disconn_peers_res: Result<Vec<RemotePeerInfo>, String> = self.get_disconn_peers();
        if disconn_peers_res.is_err() {
            return Err("Getting disconnected peers failed".to_owned());
        }

        for p_info in disconn_peers_res.unwrap() {
            if let Some(id) = new_peer.get("id") {
                if let Value::String(id_s) = id {
                    if p_info.addr.dest_hash == *id_s {
                        let disk_clone = p_info.clone();
                        new_p = RemotePeer::new(p_info);
                        self.peers.push(new_p);

                        // add to persistant peers if new
                        return RemotePeerInfo::append_peers_to_disk(vec![disk_clone]);
                    }
                } 
                
                else {
                    return Err("incorrect value for id".to_owned()) 
                }
            }
        }
        Err("peer unknown".to_owned())
    }

    fn get_disconn_peers(&self) -> Result<Vec<RemotePeerInfo>, String> {
        let disk_peer_res: Result<Vec<RemotePeerInfo>, String> = RemotePeerInfo::load_remote_peers();
        match disk_peer_res {
            Ok(d_peers) => {
                return Ok(self.filter_disconnected(&d_peers));
            },
            Err(msg) => {
                return Err(("Failed to get remote peers from disk".to_owned()));
            }
        }
    }

    fn filter_disconnected(&self, p_infos: &Vec<RemotePeerInfo>) -> Vec<RemotePeerInfo> {
        let mut disconn_peers: Vec<RemotePeerInfo> = vec![];

        for peer_info in p_infos {
            // brute forcing, but peers in a given overlay have upper limit of 150

            let current_id = &peer_info.id.value;
            let mut found = false;

            for online_peer in &self.peers {
                if *current_id == online_peer.info.id.value {
                    found = true;
                }
            }

            if !found {
                disconn_peers.push(peer_info.clone());
            }
        }
        
        return disconn_peers;
    }
    
}