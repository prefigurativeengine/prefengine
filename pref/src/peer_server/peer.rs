
use crate::peer_server::connection;

pub struct Peer {
    pub state: PeerState,
    pub connection: connection::TcpConnection,
    pub info: PeerInfo
}

// pub struct RemotePeerInfo {
//     network_space: NetworkSide,
//     capability_type: PeerCapability,
// }





pub struct PeerInfo {
    pub p_type: PeerType,
    pub network_space: NetworkSide, 
    capability_type: PeerCapability
}

use crate::core::dir;
use std::net::Ipv4Addr;
use std::{fs, path::PathBuf, str::FromStr};
use std::path;

impl PeerInfo {
    pub fn load_remote_peers() -> Result<Vec<PeerInfo>, String> {

        match dir::get_root_file_path("peers.json") {
            Ok(peers_str) => {
                // TODO: filter out local peer
                let mut disk_peers: Vec<PeerInfo> = PeerInfo::get_peers_from_disk(peers_str)
                    .expect("Failed to parse peers json.");
                return Ok(disk_peers);
            }

            Err(err) => {
                return Err(err)
            }

        }


    }

    fn get_peers_from_disk(peer_path: PathBuf) -> Result<Vec<PeerInfo>, String> 
    {
        if (path::Path::new(&peer_path).exists()) {
            /* 
            TODO: impl file parsing

            let peer_json = fs::read_to_string(peer_path);

            if (peer_json.is_empty()) {
                return None;
            }

            let json_array: Vec<PeerInfo> =
                serde_json::from_str(&peer_json).expect("peers JSON was not well-formatted");
            
            let mut disk_peers = vec![];

            for value in json_array
            {
                let sp = PeerInfo {
                    id: value.id,
                    addr: value.addr,
                    public_key: PublicKey((ssb_id::SSB_NET_ID)),
                };

                disk_peers.push(sp);
            }
            */

            let pi1 = PeerInfo {
                p_type: PeerType::Remote,
                network_space: NetworkSide {
                    addr: PeerAddress {
                        ip: Some(Ipv4Addr::from_str("s").expect("msg"))
                        bt: None
                    }
                },
                capability_type: PeerCapability::Desktop,
            };

            return Ok(vec![pi1]);
        }

        else {
            fs::File::create(peer_path).expect("Unable to create json peers file");
            return Ok(vec![]);
        }
    }

}

pub enum PeerType {
    Remote,
    // only one local should exist
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
    ip: Option<Ipv4Addr>,
    bt: Option<String>
}

