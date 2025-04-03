
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
use std::vec;
use std::{fs, path::PathBuf};
use std::path::{self, Path};

// TODO: refactor to seperate implementations managing one remote peer and the peer group as a whole, encapsulating the collection

impl RemotePeerInfo {
    pub fn from_temps_write(temps: Vec<TempPeerInfo>) -> Result<(), String>  {
        let mut new_peers = vec![];
        for t in temps {
            new_peers.push(
                Self::new(t.dest_hash)?
            );
        }

        Self::append_peers_to_disk(new_peers)?;
        Ok(())
    }

    pub fn new(ret_hash: String) -> Result<RemotePeerInfo, String> {
        Ok(Self {
            id: PeerId { value: Self::get_next_unique_id()? },
            addr: PeerAddress {
                ip: None,
                dest_hash: ret_hash,
                bt: None
            },
            cap_type: PeerCapability::Desktop
        })
    }

    pub fn load_remote_peers() -> Result<Vec<RemotePeerInfo>, String> {
        let disk_peers = RemotePeerInfo::get_peers_from_disk("peers.json")?;
        Ok(disk_peers)
    }


    pub fn append_peers_to_disk(new_peers: Vec<RemotePeerInfo>) -> Result<(), String>  {
        let peer_path_res = dir::get_root_file_path("peers.json");
        match peer_path_res {
            Ok(peer_path) => {
                let peer_json = fs::read_to_string(&peer_path)
                    .map_err(|err| err.to_string())?;

                let mut json_array: Vec<RemotePeerInfo> = serde_json::from_str(&peer_json)
                    .map_err(|err| err.to_string())?;
                json_array.extend(new_peers);

                let json_str = serde_json::to_string(&json_array)
                    .map_err(|err| err.to_string())?;

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
        let peer_json = fs::read_to_string(peer_path)
            .map_err(|err| err.to_string())?;

        if peer_json.is_empty() {
            return Ok(vec![]);
        }

        let json_array_r =
            serde_json::from_str(&peer_json);

        if let Err(err) = json_array_r {
            return Err(err.to_string());
        }
    
        return Ok(json_array_r.unwrap());
    }

    pub fn get_next_unique_id() -> Result<u16, String> {
        let disk_peers_r = RemotePeerInfo::get_peers_from_disk("peers.json")?;
        
        Ok((disk_peers_r.len() + 1) as u16)
    }

}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct TempPeerInfo {
    pub dest_hash: String
}

impl TempPeerInfo {
    pub fn append_temp_to_disk(temp: TempPeerInfo) -> Result<(), String>  {
        let temp_json = fs::read_to_string("expected_temps.json")
            .map_err(|err| err.to_string())?;

        let mut json_array: Vec<TempPeerInfo> = serde_json::from_str(&temp_json)
            .map_err(|err| err.to_string())?;
        json_array.push(temp);

        let json_str = serde_json::to_string(&json_array)
            .map_err(|err| err.to_string())?;

        match fs::write("expected_temps.json", json_str) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string())
        }
    }

    // TODO: assumes expected_temps only has empty array or array of strings
    pub fn load_expected_temps() -> Result<Vec<TempPeerInfo>, String> {
        let temp_json = fs::read_to_string("expected_temps.json")
            .map_err(|err| err.to_string())?;

        let temp_array: Value = serde_json::from_str(&temp_json)
            .map_err(|err| err.to_string())?;

        let mut expected = vec![];
        
        for t in temp_array.as_array().unwrap() {
            let t_obj = TempPeerInfo {
                dest_hash: t.as_str().unwrap().to_owned()
            };
            expected.push(t_obj);
        }
    
        return Ok(expected);
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
    Server,
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
    pub fn new_self_peer(cap: PeerCapability, ip: Ipv4Addr, dest_hash: String) -> Result<SelfPeerInfo, String> {
        let self_dummy_path = Path::new("self_peer.dummy.json");
        let self_path = Path::new("self_peer.json");

        let dummy_str = fs::read_to_string(self_dummy_path)
            .map_err(|err| err.to_string())?;

        let mut self_map: HashMap<String, Value> = serde_json::from_str(&dummy_str)
            .map_err(|err| err.to_string())?;

        let id = RemotePeerInfo::get_next_unique_id()?;

        *self_map.get_mut("id").unwrap() = Value::Object(
            Map::from_iter([
                ("value".to_owned(), Value::Number(Number::from(id))),
        ]));

        *self_map.get_mut("addr").unwrap() = Value::Object(
            Map::from_iter([
            ("ip".to_owned(), Value::String(ip.to_string())),
            ("dest_hash".to_owned(), Value::String(dest_hash)),
            ("bt".to_owned(), Value::Null),
        ]));

        match cap {
            // TODO: impl all cases
            PeerCapability::Desktop => {
                *self_map.get_mut("cap_type").unwrap() = Value::String("Desktop".to_owned())
            },
            _ => *self_map.get_mut("cap_type").unwrap() = Value::String("Cap".to_owned())
        }

        
        *self_map.get_mut("disk").unwrap() = json!({});

        let self_str = serde_json::to_string(&self_map)
            .map_err(|err| err.to_string())?;
        
        match fs::write(self_path, self_str) {
            Ok(()) => {
                let self_info: SelfPeerInfo = serde_json::from_value(
                    serde_json::to_value(self_map).map_err(|err| err.to_string())?
                ).map_err(|err| err.to_string())?;
                
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
        
        let disk_peer = disk_peer_r?;
        Ok(disk_peer)
    }
    
    fn get_self_peer_from_disk(peer_path: &str) -> Result<SelfPeerInfo, String> 
    {
        let peer_json = fs::read_to_string(peer_path)
            .map_err(|err| err.to_string())?;
        
        let json_obj = serde_json::from_str(&peer_json)
            .map_err(|err| err.to_string())?;
        
        Ok(json_obj)
    }
}


#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct DiskInfo {
}

pub fn add_all_temp_peers() -> Result<(), String> {
    let temps = TempPeerInfo::load_expected_temps()?;
    RemotePeerInfo::from_temps_write(temps)?;
    Ok(())
}