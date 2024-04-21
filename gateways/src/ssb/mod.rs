mod ssb_id;
mod tokio_compat_fix;

use std::collections::HashMap;

use kuska_ssb::{crypto::ed25519::{PublicKey, SecretKey}, keystore::OwnedIdentity};
use kuska_sodiumoxide::crypto::{auth, sign::ed25519};

use kuska_handshake::{async_std as kuska_async_std, HandshakeComplete};

use async_std;

use log::kv::Error;
// handshake_client
use tokio::{self, io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream, UdpSocket as TokioUdpSocket}
}; // 1.37.0

use tokio_compat_fix::TokioCompatFix;


static _TCP_ENDPOINTS: HashMap<&str, &str> = HashMap::from([
    ("root", "/"),
]);


struct SSBTcpClient 
{
}

impl SSBTcpClient
{
    // async fn new(peer_infos: &Vec<SSBPeerInfo>) -> SSBTcpClient 
    // {
    //     // let mut peers: Vec<SSBPeer> = Vec::new();
    //     // for p in peer_infos {
    //     //     // TODO: maybe make a macro for annotating iterator variables

    //     //     let stream_res = TokioTcpStream::connect(p.addr.clone()).await;
    //     //     if stream_res.is_err() {
    //     //         // TODO: add error info to peer struct
    //     //     } else {
    //     //         let new_peer = SSBPeer { 
    //     //             metadata: p.clone(), 
    //     //             stream: stream_res.unwrap() 
    //     //         };
    //     //         peers.push(new_peer);
    //     //     }
    //     // }
        
    //     // return SSBTcpClient { _peers: peers }
    // }

    // TODO: make error enum for this
    // async fn add_conn(&mut self, peer_info: &SSBPeerInfo) -> Result<(), String> 
    // { 
    //     let stream_res = TokioTcpStream::connect(peer_info.addr.clone()).await;
    //     if stream_res.is_err() {
    //         return Err("Failed to create TcpStream.".to_owned());
    //     }

    //     let new_peer = SSBPeer {
    //         metadata: peer_info.clone(), 
    //         stream: stream_res.unwrap()
    //     };
    //     self._peers.push(new_peer);
    //     return Ok(());
    // }

    
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
        
        let id_res: Result<OwnedIdentity, String> = ssb_id::get_ssb_id();
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
}

#[derive(Clone)]
struct SSBPeerInfo
{
    id: u32,
    addr: String,
    public_key: PublicKey,
    hs_info: Option<HandshakeCompleteFix>
}


fn get_peers_from_disk() -> Option<Vec<SSBPeer>> 
{ 

}

struct SSBTcpServer 
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

        let peer_result: Option<Vec<SSBPeerInfo>> = get_peers_from_disk();
        let peers_info = vec![];
        if let Some(some_peers) = peer_result {
            peers_info = some_peers;
        }

        let peers: Vec<SSBPeer> = SSBTcpServer::connect_peers(peers_info).await;


        SSBTcpServer::handshake_peers(&mut peers);
        return Ok(
            SSBTcpServer {
            _listener: tcp_result.unwrap(),
            _peers: peers
            }
        )
    }

    async fn handshake_peers(peers: &mut Vec<SSBPeer>)
    {
        for p in peers {
            if (p.metadata.hs_info.is_none()) {
                let hs_result: Result<HandshakeComplete, String> = SSBTcpClient::initiate_handshake(
                    p, 
                    false
                ).await;

                if hs_result.is_err() {
                    // see TODO above SSBTcpClient def
                } else {
                    // let mut_p = client.get_mut_peer(ind);
                    // mut_p.metadata.is_handshaked = true;
                    let hs: HandshakeCompleteFix = HandshakeCompleteFix::clone_org_to_fix(
                        hs_result.unwrap()
                    );
                    p.metadata.hs_info = Some(hs);
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
                    stream: stream_res.unwrap() 
                };
                peer_streams.push(new_peer);
            }
        }
        return peer_streams;
    }
}


enum SSBDiscoveryMethod
{
    // only invitecode will be implemented for now
    LANBroadcast,
    InviteCode,
    BluetoothBroadcast
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
