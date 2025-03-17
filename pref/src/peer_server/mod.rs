use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use std::{fs, thread}; 

mod peer;
use peer::Peer;
use peer::PeerInfo;
use peer::PeerType;
use peer::PeerCapability;
use serde_json::Value;

mod connection;

pub mod db;

pub mod ret_util;

const RET_URL: &str = "127.0.0.1:3502";
const PREF_URL: &str = "127.0.0.1:3501";


pub struct Server {
    ret_api_listener: net::TcpListener,
    ret_api_conn: net::TcpStream,
    peers: Vec<Peer>,
}

const FO_RECONNECT_ACTION: &str = "fo_reconnect";
const SEND_ACTION: &str = "send";

use serde_json::{Result};

impl Server {
    pub fn new() -> Server {
        let server = Server {
            ret_api_listener: TcpListener::bind(PREF_URL)
                .expect("Could not start the reticulum listener"),
            ret_api_conn: TcpStream::connect(RET_URL)
                .expect("Could not connect to reticulum"),
            peers: Vec::new(),
        };

        server
    }

    pub fn start(&self) {
        self.peer_connect_all();
        thread::spawn(|&self| {
			&self.ret_listen();
		});
    }

    pub fn send_db_change(&mut self, change: String) -> Result<(), String> {
        let mut change_map = HashMap::new();
        change_map.insert("action".to_owned(), "send".to_owned());
        change_map.insert("change".to_owned(), change);

        let json_s = Server::format_for_ret(None, SEND_ACTION, Some(change_map));
        match self.ret_send(json_s) {
            Ok(size) => Ok(()),
            Err(err) => return Err(err.to_string())
        }
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
            return Err(Error::new(ErrorKind::Other, "Cannot connect to local peer"))
        }
        
        // TODO: run through a list of connection tactics according to values in peerinfo    
        let json_s = Server::format_for_ret(Some(peer.id.parent_id), FO_RECONNECT_ACTION, None);
        let res = self.ret_send(json_s);
        return res;
    }

    fn format_for_ret(id: Option<String>, action: &str, data: Option<HashMap<String, String>>) -> String {
        let mut hm_dto: HashMap<String, String> = HashMap::new();

        if let Some(id_val) = id {
            hm_dto.insert("id".to_owned(), id_val);
        }
        
        hm_dto.insert("action".to_owned(), action.to_owned());
        
        if matches!(data, Some(_)) {
            hm_dto.extend(data.unwrap());
        }

        return serde_json::to_string(hm_dto);
    }

    fn ret_send(&mut self, data: String) -> Result<usize, std::io::Error> {
        return self.ret_api_conn.write(data.as_bytes());
    }

    
    fn ret_listen(&self) {
        for stream in self.ret_api_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut buf = String::new();

                    match stream.read_to_string(&mut buf) {
                        Ok(usize) => {
                            Server::dispatch_ret_resp(buf);
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

    fn dispatch_ret_resp(resp: String) -> Result<(), String> {
        // HACK
        let new_peer = "{""action"":""new_peer";
        let resc_fin = "{""action"":""resc_fin";

        match &resp[0..19] {
            new_peer => {
                let serde_res: Result<HashMap<String, Value>, Error> = serde_json::from_str(&resp);
                if matches!(serde_res, Some(_)) {
                    let serde_j = serde_res.unwrap();
                    if self.check_peer_req(serde_j) {
                        self.add_peer(serde_j);
                    }
                } else {
                    log::error!("Failed to decode ret proxy message into json: {}", serde_res.unwrap());
                    Err("Failed to decode ret proxy message into json".to_owned())
                }
            },
            resc_fin => {
                peer_db::process_remote_change(resp);
            }
        }
    }

    fn check_peer_req(&self, resp: HashMap<String, Value>) -> bool {
        for peer in self.get_disconn_peers() {
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
            return Err("Getting disconnected peers failed".to_owned());
        }

        for p_info in disconn_peers_res.unwrap() {
            if let Some(id) = new_peer.get("id") {
                if p_info.id.child_dest_id == id {
                    new_p = Peer::new(p_info);
                    self.peers.push(new_p);
                }
            }
        }
        
        // add to persistant peers if new
        PeerInfo::append_peers_to_disk(vec![new_p.info]);
    }

    fn get_disconn_peers(&self) -> Result<Vec<PeerInfo>, String> {
        let disk_peer_res: Result<Vec<PeerInfo>, String> = PeerInfo::load_remote_peers();
        match disk_peer_res {
            Ok(d_peers) => {
                Ok(self.filter_pinfo_for_disconn(d_peers));
            }
            Err(msg) => {
                Err(("Failed to get remote peers from disk".to_owned()));
            }
        }
    }

    fn filter_pinfo_for_disconnected(self, p_infos: &Vec<PeerInfo>) -> Vec<PeerInfo> {
        let mut disconn_peers: Vec<PeerInfo> = vec![];

        for peer_info in p_infos {
            // brute forcing, but peers in a given overlay have upper limit of 150

            let current_id = peer_info.id.parent_id;
            let found: bool;

            for online_peer in self.peers {
                if current_id == online_peer.id.parent_id {
                    found = true;
                }
            }

            if !found {
                disconn_peers.push(peer_info.to_owned());
            }
        }
        
        return disconn_peers;
    }

    // fn on_new_peer_connect(&self, stream: TcpStream) {
    //     // check if a valid ip addr
    //     if self.valid_peer_addrs.contains(stream.peer_addr()) {
    //         // add as connection
    //     } else {
    //         // if not a valid ip, peek msg to see full of identity of remote 
    //         stream.read_timeout();
    //         // if invalid identity, drop

    //         // if valid, add as connection and update ip addr
    //     }
    // }

    fn try_traversal_methods() {

    }

    fn handle_conn_failure() {

    }

}
