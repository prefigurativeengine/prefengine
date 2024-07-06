pub mod ssb_id;
mod tokio_compat_fix;

use std::collections::HashMap;

use kuska_ssb::{crypto::ed25519::{PublicKey}, keystore::OwnedIdentity};
use kuska_sodiumoxide::crypto::{auth};

use kuska_handshake::{async_std as kuska_async_std, HandshakeComplete};

use log::kv::Error;
// handshake_client
use tokio::{self, net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream, UdpSocket as TokioUdpSocket}
}; // 1.37.0

use tokio_compat_fix::TokioCompatFix;
use std::fs;
use std::path;
use serde_json;
use std::env;

// static _TCP_ENDPOINTS: HashMap<&str, &str> = HashMap::from([
//     ("root", "/"),
// ]);


struct SSBTcpClient 
{
}

impl SSBTcpClient
{
    // TODO: make error enum for this
    async fn initiate_handshake(
        peer: &mut SSBPeer,
        use_ssb_net: bool
    ) -> Result<HandshakeComplete, String> 
    {
        use auth::Key;

        let net_id: Key;
        if (use_ssb_net) {
            net_id = Key(ssb_id::SSB_NET_ID);
        }
        else {
            net_id = Key(ssb_id::GATE_NET_ID);
        }
        
        let id_res: Result<OwnedIdentity, String> = ssb_id::get_ssb_id().await;
        if id_res.is_err() {
            return Err("Failed to get ssb id.".to_owned());
        }
        let client_id = id_res.unwrap();

        let self_pk = peer.metadata.public_key;
        let server_pk: PublicKey = self_pk.clone();

        let mut async_std_adapter: TokioCompatFix<&mut TokioTcpStream> = TokioCompatFix { 
            0: &mut peer.stream
        };

        // returns custom Result type from kuska_ssb, either HandshakeComplete or kuska_ssb err
        let handshake_res = kuska_async_std::handshake_client(
            &mut async_std_adapter, 
            net_id, 
            client_id.pk, 
            client_id.sk, 
            server_pk
        ).await;

        if handshake_res.is_err() {
            return Err("Failed to perform handshake.".to_owned());
        }

        return Ok(handshake_res.unwrap());
    }
}


// make SSBPeer methods for accessing stream
struct SSBPeer 
{
    pub metadata: SSBPeerInfo,
    stream: TokioTcpStream,
    hs_info: Option<HandshakeCompleteFix>
}


use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize)]
struct SSBPeerInfo
{
    id: u32,
    addr: String,
    public_key: PublicKey,
}


fn get_peer_file_path() -> path::PathBuf
{
    let mut json_path = env::current_dir()
            .expect("Unable to read current working directory");

    json_path.push("peers.json");
    return json_path;
}

fn get_peer_file_str() -> String
{
    let json_path = get_peer_file_path();

    return fs::read_to_string(json_path)
        .expect("Unable to read from peers file");
}


fn add_peer_to_disk(peer: &SSBPeerInfo)
{
    let peers_str = get_peer_file_str();

    let mut json_array: Vec<SSBPeerInfo> =
        serde_json::from_str(&peers_str).expect("peers JSON was not well-formatted");

    json_array.push(peer.clone());

    let serialized = serde_json::to_string(&json_array).unwrap();

    fs::write(get_peer_file_path(), serialized).expect("Unable to write to peers file");
}


fn get_peers_from_disk() -> Option<Vec<SSBPeerInfo>> 
{
    let mut json_path = get_peer_file_path();

    if (path::Path::new(&json_path).exists()) {
        let peers_str = get_peer_file_str();

        if (peers_str.is_empty()) {
            return None;
        }

        let json_array: Vec<SSBPeerInfo> =
            serde_json::from_str(&peers_str).expect("peers JSON was not well-formatted");
        
        let mut disk_peers = vec![];

        // vm - 192.132.123.233:3501
        for value in json_array
        {
            let sp = SSBPeerInfo {
                id: value.id,
                addr: value.addr,
                public_key: PublicKey((ssb_id::SSB_NET_ID)),
            };

            disk_peers.push(sp);
        }
        

        return Some(disk_peers);
    }

    else {
        fs::File::create(json_path).expect("Unable to create json peers file");
        return None;
    }
}

pub struct SSBTcpServer 
{
    _listener: TokioTcpListener,
    //_client: SSBTcpClient,
    _peers: Vec<SSBPeer>
}

impl SSBTcpServer 
{
    pub async fn new() -> Result<SSBTcpServer, String>
    {
        let port: &str = ":3501";
        let public_addr: String = "0.0.0.0".to_owned() + port;

        let tcp_result = TokioTcpListener::bind(public_addr).await;
        if tcp_result.is_err() {
            return Err("Failed to bind TcpListener.".to_owned());
        }

        //let peer_result: Option<Vec<SSBPeerInfo>> = get_peers_from_disk();
        let mut peers_info = vec![];
        // if let Some(some_peers) = peer_result {
        //     peers_info = some_peers;
        // }

        let mut peers: Vec<SSBPeer> = SSBTcpServer::connect_peers(peers_info).await;


        SSBTcpServer::handshake_peers(&mut peers);
        return Ok(
            SSBTcpServer {
            _listener: tcp_result.unwrap(),
            _peers: peers
            }
        )
    }

    fn add_peer(&mut self, info: SSBPeerInfo, stream: TokioTcpStream, hs_info: HandshakeCompleteFix)
    {
        add_peer_to_disk(&info);
        
        let stream_peer = SSBPeer { metadata: info, stream: stream, hs_info: Some(hs_info) };
        self._peers.push(stream_peer);
    }

    async fn handshake_peers(peers: &mut Vec<SSBPeer>)
    {
        for p in peers {
            if (p.hs_info.is_none()) {
                let hs_result: Result<HandshakeComplete, String> = SSBTcpClient::initiate_handshake(
                    p, 
                    false
                ).await;

                if hs_result.is_err() {
                    // see TODO above SSBTcpClient def
                } else {
                    let hs: HandshakeCompleteFix = HandshakeCompleteFix::clone_org_to_fix(
                        hs_result.unwrap()
                    );
                    p.hs_info = Some(hs);
                }
            }
        }
    }

    async fn connect_peers(peer_infos: Vec<SSBPeerInfo>) -> Vec<SSBPeer>
    {
        let mut peer_streams = vec![];
        for p in peer_infos {
            // TODO: maybe make a macro for annotating iterator variables

            let stream_res = TokioTcpStream::connect(p.addr.clone()).await;
            if stream_res.is_err() {
                // TODO: add error info to peer struct
            } else {
                let new_peer = SSBPeer { 
                    metadata: p.clone(), 
                    stream: stream_res.unwrap(),
                    hs_info: None
                };
                peer_streams.push(new_peer);
            }
        }
        return peer_streams;
    }
}

use std::net::SocketAddr;

use self::tokio_compat_fix::HandshakeCompleteFix;
async fn send_udp(data: &[u8], dest_addr: SocketAddr) -> Result<(), Error>
{
    let addr: &str = "0.0.0.0:3502";
    let socket: TokioUdpSocket = TokioUdpSocket::bind(addr).await?;

    socket.send_to(data, &dest_addr).await?;
    return Ok(());
}

async fn recv_udp() -> Result<(Vec<u8>, SocketAddr), Error>
{
    let addr: &str = "0.0.0.0:3502";
    let socket: TokioUdpSocket = TokioUdpSocket::bind(addr).await?;

    let mut buf = vec![0u8; 32];
    let res_tuple: (usize, SocketAddr) = socket.recv_from(&mut buf).await?;
    return Ok((buf, res_tuple.1));
}
