
use std::net::TcpStream;

use crate::peer_server::peer;
pub struct TcpConnection {
    peer_ref: &peer::Peer,
    strengh: TcpStrength,
    visibilities: Vec<u32>,
    net_type: NetType,
    tcp_stream: TcpStream
}


impl TcpConnection {
    pub fn new(peer: &peer::Peer, stm: TcpStream) -> TcpConnection {
        return TcpConnection {
            peer,
            TcpStrength { },
            vec![0, 2],
            NetType { },
            stm
        }
    }

    pub fn p_write() {

    }

    pub fn p_read() {

    }
}



struct TcpStrength {
    /* 
    msg_readiness: MsgReadiness,
    signal_quality: SignalQuality */
}



enum NetType {
    Internet,
    CustomNet
}


struct MsgReadiness {
    _value: u32
}

struct SignalQuality {
    _value: u32
}

