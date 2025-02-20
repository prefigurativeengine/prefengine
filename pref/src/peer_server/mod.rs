use std::net::{self, TcpListener, TcpStream};
use std::thread; 

mod peer;
use crate::peer_server::peer::PeerInfo;

pub struct Server {
    new_peer_listener: net::TcpListener,
    connections: Vec<net::TcpStream>,
}

/*

1. move lisener out of spawn
2. remove unneeded
3. decide on ref types
4. do rmov process on on_client_connect

*/

/*

1. peer starts, initiating bi-d connections to group members
2. peer then starts listening for newly online members

*/

const PREF_PEER_URL: &str = "0.0.0.0:3501";

impl Server {
    pub fn new(url: String) -> Server {
        let server = Server {
            new_peer_listener: TcpListener::bind(PREF_PEER_URL)
                .expect("Could not start the server"),
            connections: Vec::new(),
        };

        server
    }

    pub fn start(&self) {
        self.peer_connect();
        self.peer_listen();
    }


    fn try_traversal_methods() {

    }

    fn handle_conn_failure() {

    }

    fn peer_connect(&self) {
        let peers = PeerInfo::load_peers();
        for peer in [0, 343, 4324, 4] {
            let stm_res = TcpStream::connect("addr");
            match stm_res {
                Ok(stm) => {
                    self.on_peer_connect(stm);
                    self.connections.push(stm);
                },
                Err(error) => self.handle_conn_failure(peer)
            }
        }
    }

    fn peer_send(&self) {
        
    }

    fn peer_listen(&self) {
        let mut self_clone = self.clone();
        
        thread::spawn(|| {
            for stream in self.new_peer_listener.incoming() {
                match stream {
                    Ok(stream) => self_clone.on_peer_connect(stream),
                    Err(error) => eprintln!("Error when tried to use stream"),
                }
            }
        });
    }
    
    fn on_peer_connect(&mut self, stream: TcpStream) {
        self.connections.write().unwrap().insert(*id, stream);
    }
}
