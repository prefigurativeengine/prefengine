pub struct Peer {
    state: PeerState,
    info: PeerInfo
}

// pub struct RemotePeerInfo {
//     network_space: NetworkSide,
//     capability_type: PeerCapability,
// }

pub struct PeerInfo {
    p_type: PeerType
}

impl PeerInfo {
    pub fn load_peers() -> Result<Vec<PeerInfo>, String> {
        
    }
}

enum PeerType {
    Remote { network_space: NetworkSide, capability_type: PeerCapability },
    Local { network_space: NetworkSide, capability_type: PeerCapability, local_space: LocalSide, }
}


enum PeerCapability {

}

enum PeerState {

}

struct LocalSide {

}

struct NetworkSide {

}




