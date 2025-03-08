use std::net::{self, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use std::{fs, thread}; 

mod peer;
use peer::Peer;

use crate::peer_server::peer::{self, *};

mod connection;
use crate::peer_server::connection as conn;

use crate::core::{self, *};
use configparser::ini::Ini;

const RET_URL: &str = "127.0.0.1:3502";

pub fn gen_config(
    c_type: peer::PeerCapability, 
    bt: bool, 
    log_level: u32, 
    auth_pass: String, 
    ipv6_addr: Option<Ipv6Addr>) -> Result<(), String> {
    // Get needed info; c type, available socket interfaces. assumes a tcp interface is connected to internet.

    let mut config = Ini::new();
    config.load("reticulum_dummy_config.conf")?;
    
    match c_type {
        peer::PeerCapability::Desktop => {
            config.set("reticulum", "enable_transport", Some("No"));
            config.set("reticulum", "share_instance", Some("No"));
        }

        peer::PeerCapability::Server | peer::PeerCapability::PtpRelay => {
            config.set("reticulum", "enable_transport", Some("Yes"));
        }
    }

    // TODO: when loglevel is actually implemented for prefengine, make the prefengine loglevel match reticulum's tiers
    config.set("logging", "loglevel", Some(log_level));

    // TODO: reticulum only supports hardcoded auth passphrases on file, later on this needs to be dynamic and not just based 
    // off a file
    config.set("TCP Server Interface", "passphrase", Some(auth_pass));

    match ipv6_addr {
        Some(addr) => {
            config.set("TCP Server Interface", "listen_ip", Some(addr));
            config.set("TCP Client Interface", "target_host", Some(addr));    
        }
    }

    return config.write("reticulum_config.conf");
}


pub struct Server {
    ret_api_listener: net::TcpListener,
    ret_api_conn: net::TcpStream,
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


impl Server {
    pub fn new(url: String) -> Server {
        let server = Server {
            ret_api_listener: TcpListener::bind(PREF_PEER_URL)
                .expect("Could not start the reticulum listener"),
            ret_api_conn: TcpStream::connect(RET_URL)
                .expect("Could not connect to reticulum");
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

    fn peer_connect_all(&mut self) -> Result<(), String> {
        if let Ok(peers) = PeerInfo::load_remote_peers() {
            for peer in peers {
                self.peer_connect(peer);
            }
            return Ok(());
        }
        else {
            return Err("Failed to load peers".to_owned())
        }
    }

    fn peer_connect(&mut self, peer: PeerInfo) -> Result<(), &str> {
        if matches!(peer.p_type, PeerType::Local { local_space: _ }) {
            return Err(("Local peer cannot be connected to."))
        }
        
        match peer.network_space.addr.ip {
            None => {
                return Err(("peer_connect not for bluetooth."))
            },
            Some(ip) => {

                // TODO: run through a list of connection tactics according to values in peerinfo

                self.reticulum_send
                match ret_api_conn {
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
