use std::net::{self, Ipv4Addr, TcpListener, TcpStream};
use std::thread; 

mod peer;
use peer::Peer;

use crate::peer_server::peer::PeerInfo;
use crate::peer_server::peer::PeerType;

mod connection;
use crate::peer_server::connection;


pub struct Server {
    new_peer_listener: net::TcpListener,
    peers: Vec<Peer>
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
            peers: Vec::new(),
        };

        server
    }

    pub fn start(&self) {
        self.peer_connect_all();
        self.peer_listen();
    }


    fn try_traversal_methods() {

    }

    fn handle_conn_failure() {

    }

    fn peer_connect_all() -> Result<(), &str> {
        if let Ok(peers) = PeerInfo::load_remote_peers() {
            for peer in peers {
                peer_connect(peer);
            }
            return Ok(());
        }
        else {
            return Err("Failed to load peers")
        }
    }

    fn peer_connect(&self, peer: PeerInfo) -> Result<(), &str> {
        if peer.p_type == PeerType::Local {
            return Err(("Local peer cannot be connected to."))
        }

        let stm_res = TcpStream::connect(peer.network_space.addr);
        match stm_res {
            Ok(stm) => {
                let tcp_conn = connection::TcpConnection::new(peer, stm);

                // TODO: impl communication of state
                let dummy_state = PeerState::Active;

                self.peers.push(
                    Peer {
                        state: dummy_state,
                        connection: tcp_conn,
                        info: peer
                    }
                );
            },
            Err(error) => self.handle_conn_failure(peer)
        }
    }

    fn peer_send(&self) {
        
    }

    fn peer_listen(&self) {
        thread::spawn(|| {
            for stream in self.new_peer_listener.incoming() {
                match stream {
                    Ok(stream) => self.on_new_peer_connect(stream),
                    Err(error) => eprintln!("Error when tried to use stream"),
                }
            }
        });
    }
    
    fn on_new_peer_connect(&self, stream: TcpStream) {
        // check if a valid ip addr
        if self.valid_peer_addrs.contains(stream.peer_addr()) {
            // add as connection
        } else {
            // if not a valid ip, peek msg to see full of identity of remote 
            stream.read_timeout();
            // if invalid identity, drop

            // if valid, add as connection and update ip addr
        }
    }
}
