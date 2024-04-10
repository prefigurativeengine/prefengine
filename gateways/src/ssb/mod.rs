pub mod ssb_id;


use tokio; // 1.37.0

struct HandshakeHandler {}

struct SSBTcpResponder {}

struct SSBTcpServer 
{
    _listener: tokio::net::TcpListener,
    _handshaker: HandshakeHandler,
    _endpoints: HashMap<&str, &str>,
    _client: SSBTcpResponder
}

// impl SSBServer 
// {
//     pub fn new()
//     {

//     }
// }
