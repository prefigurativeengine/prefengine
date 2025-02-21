
use crate::peer_server::connection;

pub struct Peer {
    state: PeerState,
    connection: connection::TcpConnection,
    info: PeerInfo
}

// pub struct RemotePeerInfo {
//     network_space: NetworkSide,
//     capability_type: PeerCapability,
// }





pub struct PeerInfo {
    p_type: PeerType,
    pub network_space: NetworkSide, 
    capability_type: PeerCapability
}

use crate::core::dir;
use std::{fs, net::Ipv4Addr};

impl PeerInfo {
    pub fn load_peers() -> Result<Vec<PeerInfo>, &str> {
        let peer_path = dir::get_root_file_path("peers.json");

        match fs::read_to_string(peer_path) {
            Ok(peers_str) => {
                let disk_peers: Vec<PeerInfo> = self.parse_fpeers(peers_str)
                    .expect("Failed to parse peers json.");
                return Ok(disk_peers);
            }

            Err(err) => {
                return Err("Failed to read peers json.")
            }

        }


    }

    fn parse_fpeers(peers_str: String) -> Result<Vec<PeerInfo>, String> {
        // impl this, will be json for now but might later be stored in encrypted db
    }
}

enum PeerType {
    Remote,
    Local { local_space: LocalSide }
}

// TODO: impl all of peer model


enum PeerCapability {

}

enum PeerState {

}

struct LocalSide {

}

struct NetworkSide {
    pub addr: PeerAddress
}

// make it so that either or both of ip and bluetooth can be set but one must be set
enum PeerAddress {
    Ip(Ipv4Addr),
    Bluetooth(String)
}

