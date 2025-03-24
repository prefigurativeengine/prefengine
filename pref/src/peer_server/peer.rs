
pub struct RemotePeer {
    pub state: PeerState,
    pub info: RemotePeerInfo
}

impl RemotePeer {
    pub fn new(param_info: RemotePeerInfo) -> RemotePeer {
        RemotePeer {
            state: PeerState::Active,
            info: param_info
        }
    }
}



#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct RemotePeerInfo {
    pub id: PeerId,
    pub addr: PeerAddress,
    cap_type: PeerCapability,
}

use serde_json::{json, Value};

use crate::core::dir;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::{fs, path::PathBuf};
use std::path::{self, Path};

// TODO: refactor to make peer info management like PeerStore (encapsulated collections)

impl RemotePeerInfo {
    pub fn load_remote_peers() -> Result<Vec<RemotePeerInfo>, String> {
        let disk_peers_r = RemotePeerInfo::get_peers_from_disk("peers.json");
        if let Err(err) = disk_peers_r {
            return Err(err);
        }
        
        let disk_peers = disk_peers_r.unwrap();
        Ok(disk_peers)
    }


    pub fn append_peers_to_disk(new_peers: Vec<RemotePeerInfo>) -> Result<(), String>  {
        let peer_path_res = dir::get_root_file_path("peers.json");
        match peer_path_res {
            Ok(peer_path) => {
                let peer_json_r = fs::read_to_string(&peer_path);
                if let Err(err) = peer_json_r {
                    return Err(err.to_string()) 
                }

                let peer_json = peer_json_r.unwrap();
                let json_array_r: Result<Vec<RemotePeerInfo>, serde_json::Error> = serde_json::from_str(&peer_json);
                if let Err(err) = json_array_r {
                    return Err(err.to_string()) 
                }

                let mut json_array: Vec<RemotePeerInfo> = json_array_r.unwrap();
                json_array.extend(new_peers);

                let json_str_r = serde_json::to_string(&json_array);
                if let Err(err) = json_str_r {
                    return Err(err.to_string()) 
                }

                let json_str = json_str_r.unwrap();
                match fs::write(peer_path, json_str) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(err.to_string())
                }
            }
            Err((msg)) => {
                return Err("Failed to get remote peers from disk".to_owned());
            }
        }
    }


    fn get_peers_from_disk(peer_path: &str) -> Result<Vec<RemotePeerInfo>, String> 
    {
        let peer_json_r = fs::read_to_string(peer_path);
        if let Err(err) = peer_json_r {
            return Err(err.to_string()) 
        }

        let json_array_r =
            serde_json::from_str(&peer_json_r.unwrap());

        if let Err(err) = json_array_r {
            return Err(err.to_string());
        }
    
        return Ok(json_array_r.unwrap());
    }

    pub fn get_next_unique_id() -> Result<u16, String> {
        let disk_peers_r = RemotePeerInfo::get_peers_from_disk("peers.json");
        if let Err(err) = disk_peers_r {
            return Err(err);
        }
        
        Ok((disk_peers_r.unwrap().len() + 1) as u16)
    }

}

// TODO: impl all of peer model

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct PeerId {
    // reticulum dest hashes will be used for identifying peers in other overlays 
    pub value: u16,
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub enum PeerCapability {
    Desktop,
    Mobile,
    Client,
    RadioRelay,
    PtpRelay
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
enum PeerState {
    Active,
    Passive,
    Off
}

// TODO: make it so that either or both of ip and bluetooth can be set but one must be set
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct PeerAddress {
    pub ip: Option<IpAddr>,
    pub dest_hash: String,
    pub bt: Option<String>
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct SelfPeerInfo {
    pub id: PeerId,
    pub addr: PeerAddress,
    pub cap_type: PeerCapability,
    pub disk: DiskInfo
}

use serde_json::{Error as s_Error, Number, Map};
impl SelfPeerInfo {
    // TODO: clean this function up, and support ipv6
    pub fn construct_self_peer(cap: PeerCapability, ip: Ipv4Addr) -> Result<SelfPeerInfo, String> {
        let self_path = Path::new("self_peer.dummy.json");

        let dummy_str_r = fs::read_to_string(self_path);
        if let Err(err) = dummy_str_r {
            return Err(err.to_string());
        }
        let dummy_str = dummy_str_r.unwrap();

        let self_map_r: Result<HashMap<String, Value>, s_Error> = serde_json::from_str(&dummy_str);
        if let Err(err) = self_map_r {
            return Err(err.to_string());
        }

        let mut self_map = self_map_r.unwrap();

        let id_r = RemotePeerInfo::get_next_unique_id();
        if let Err(err) = id_r {
            return Err(err);
        }

        let id = id_r.unwrap();
        *self_map.get_mut("id").unwrap() = Value::Number(
            Number::from(id)
        );

        *self_map.get_mut("addr").unwrap() = Value::Object(Map::from_iter([
            ("ip".to_owned(), Value::String(ip.to_string())),
            ("bt".to_owned(), Value::Null),
        ]));

        let cap_str_r = serde_json::to_string(&cap);
        if let Err(err) = cap_str_r {
            return Err(err.to_string());
        }

        let cap_str = cap_str_r.unwrap();

        *self_map.get_mut("cap_type").unwrap() = Value::String(cap_str);
        *self_map.get_mut("disk").unwrap() = json!({});

        let self_str_r = serde_json::to_string(&self_map);
        if let Err(err) = self_str_r {
            return Err(err.to_string());
        }
        
        match fs::write(self_path, self_str_r.unwrap()) {
            Ok(()) => {
                let self_info: SelfPeerInfo = serde_json::from_value(serde_json::to_value(self_map).unwrap()).unwrap();
                Ok(self_info)        
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub fn load_self_peer() -> Result<SelfPeerInfo, String> {
        let self_peer_exists = Path::new("self_peer.json").exists();

        let disk_peer_r = if self_peer_exists {
            SelfPeerInfo::get_self_peer_from_disk("self_peer.json")
        } else {
            SelfPeerInfo::get_self_peer_from_disk("self_peer.dummy.json")
        };

        if let Err(err) = disk_peer_r {
            return Err(err);
        }
        
        let disk_peer = disk_peer_r.unwrap();
        Ok(disk_peer)
    }
    
    fn get_self_peer_from_disk(peer_path: &str) -> Result<SelfPeerInfo, String> 
    {
        let peer_json_r = fs::read_to_string(peer_path);
        if let Err(err) = peer_json_r {
            return Err(err.to_string());
        }

        let peer_json = peer_json_r.unwrap();
        let json_obj_r = serde_json::from_str(&peer_json);
        if let Err(err) = json_obj_r {
            return Err(err.to_string());
        }
        
        Ok(json_obj_r.unwrap())
    }
}


#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct DiskInfo {
}