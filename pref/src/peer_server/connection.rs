
use std::net::TcpStream;


pub struct TcpConnection {
    strength: TcpStrength,
    visibles: Vec<u32>,
    net_type: NetType,
    tcp_stream: TcpStream
}


impl TcpConnection {
    pub fn new(stm: TcpStream) -> TcpConnection {
        return TcpConnection {
            strength: TcpStrength { },
            visibles: vec![0, 2],
            net_type: NetType::Internet,
            tcp_stream: stm
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
    signal_quality: SignalQuality 
    */
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

