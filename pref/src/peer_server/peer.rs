
// TODO: seperate mutable network info from peerinfo
pub struct Peer {
    pub state: PeerState,
    pub info: PeerInfo
}

impl Peer {
    pub fn new(param_info: PeerInfo) -> Peer {
        Peer {
            state: PeerState::Active,
            info: param_info
        }
    }
}

// pub struct RemotePeerInfo {
//     network_space: NetworkSide,
//     capability_type: PeerCapability,
// }





pub struct PeerInfo {
    pub id: PeerId,
    pub p_type: PeerType,
    pub network_space: NetworkSide, 
    capability_type: PeerCapability
}

pub struct PeerId {
    pub parent_id: String,
    pub child_dest_id: String
}

use crate::core::dir;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::{fs, path::PathBuf};
use std::path;

impl PeerInfo {
    pub fn load_remote_peers() -> Result<Vec<PeerInfo>, String> {

        match dir::get_root_file_path("peers.json") {
            Ok(peers_str) => {
                // TODO: ensure to filter out local peer
                let mut disk_peers: Vec<PeerInfo> = PeerInfo::get_peers_from_disk(peers_str)
                    .expect("Failed to parse peers json.");
                return Ok(disk_peers);
            }

            Err(err) => {
                return Err(err)
            }

        }
    }

    pub fn load_self_peer() -> Result<PeerInfo, String> {
        match dir::get_root_file_path("self_peer.json") {
            Ok(peer_p) => {
                let mut disk_peer: PeerInfo = PeerInfo::get_self_peer_from_disk(peer_p)
                    .expect("Failed to parse peer json.");
                return Ok(disk_peer);
            }

            Err(err) => {
                return Err(err)
            }

        }
    }

    fn get_self_peer_from_disk(peer_path: PathBuf) -> Result<PeerInfo, String> 
    {
        if (path::Path::new(&peer_path).exists()) {
            /* 
            TODO: unhack this

            let peer_json = fs::read_to_string(peer_path);

            if (peer_json.is_empty()) {
                return None;
            }

            let json_obj: PeerInfo =
                serde_json::from_str(&peer_json).expect("peers JSON was not well-formatted");

            }
            */

            let pi1 = PeerInfo {
                id: PeerId { parent_id: "test".to_owned(), child_dest_id: "test".to_owned() },
                p_type: PeerType::Local { local_space: ( LocalSide {  } ) },
                network_space: NetworkSide {
                    addr: PeerAddress {
                        ip: Some(IpAddr::V4(Ipv4Addr::from_str("s").expect("msg"))),
                        bt: None
                    }
                },
                capability_type: PeerCapability::Desktop,
            };

            return Ok(pi1);
        }

        else {
            fs::File::create(peer_path).expect("Unable to create json peers file");
            return Err("Self peer file doesn't exist".to_owned());
        }
    }


    pub fn append_peers_to_disk(new_peers: Vec<PeerInfo>) -> Result<(), String>  {
        let peer_path_res = dir::get_root_file_path("peers.json");
        match peer_path_res {
            Ok(peer_path) => {
                let peer_json_r = fs::read_to_string(peer_path);
                match peer_json_r {
                    Ok(v) => {},
                    Err(e) => { 
                        return Err(e.to_string()) 
                    }
                }

                let peer_json = peer_json_r.unwrap();

                let json_array_r: Result<Vec<PeerInfo>, serde_json::Error> = serde_json::from_str(&peer_json);
                match json_array_r {
                    Ok(v) => {},
                    Err(e) => { return Err(e.to_string()) }
                }

                let json_array: Vec<PeerInfo> = json_array_r.unwrap();
                json_array.extend(new_peers);

                let json_str_r = serde_json::to_string(&json_array);
                match json_str_r {
                    Ok(v) => {},
                    Err(e) => { return Err(e.to_string()) }
                }

                let json_str = json_str_r.unwrap();
                match fs::write(peer_path, json_str) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(err.to_string())
                }
            }
            Err((msg)) => {
                log::error!(msg);
                return Err("Failed to get remote peers from disk".to_owned());
            }
        }
    }


    fn get_peers_from_disk(peer_path: PathBuf) -> Result<Vec<PeerInfo>, String> 
    {
        if (path::Path::new(&peer_path).exists()) {
            let peer_json = fs::read_to_string(peer_path);

            let json_array: Vec<PeerInfo> =
                serde_json::from_str(&peer_json).expect("peers JSON was not well-formatted");

            // let pi1 = PeerInfo {
            //     p_type: PeerType::Remote,
            //     network_space: NetworkSide {
            //         addr: PeerAddress {
            //             ip: Some(IpAddr::V4(Ipv4Addr::from_str("s").expect("msg")),
            //             bt: None
            //         }
            //     },
            //     capability_type: PeerCapability::Desktop,
            // };

            return Ok(json_array);
        }

        else {
            fs::File::create(peer_path).expect("Unable to create json peers file");
            return Ok(vec![]);
        }
    }

}

pub enum PeerType {
    Remote,
    Local { local_space: LocalSide }
}

// TODO: impl all of peer model


pub enum PeerCapability {
    Desktop,
    Mobile,
    Server,
    RadioRelay,
    PtpRelay
}

enum PeerState {
    Active,
    Passive,
    Off
}

struct LocalSide {

}

struct NetworkSide {
    pub addr: PeerAddress
}

// TODO: make it so that either or both of ip and bluetooth can be set but one must be set
struct PeerAddress {
    ip: Option<IpAddr>,
    bt: Option<String>
}

