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
    _peers: Vec<SSBPeer>
}

impl SSBTcpClient
{
    fn new(peer_infos: &Vec<SSBPeerInfo>) -> SSBTcpClient 
    {
        let peers: Vec<SSBPeer> = Vec::new();
        for p in peer_infos {
            // TODO: maybe make a macro for annotating iterator variables

            let stream_res = TokioTcpStream::connect(p.addr).await;
            if stream_res.is_err() {
                // TODO: add error info to peer struct
            } else {
                let new_peer = SSBPeer { 
                    metadata: p.clone(), 
                    stream: stream_res.unwrap() 
                };
                peers.push(new_peer);
            }
        }
        
        return SSBTcpClient { _peers: peers }
    }

    pub fn get_peers(&self) -> &Vec<SSBPeer>
    {
        return &self._peers;
    }

    // TODO: make error enum for this
    fn add_conn(&mut self, peer_info: &SSBPeerInfo) -> Result<(), String> 
    { 
        let stream_res = TokioTcpStream::connect(peer_info.addr).await;
        if stream_res.is_err() {
            return Err("Failed to create TcpStream.".to_owned());
        }

        let new_peer = SSBPeer {
            metadata: peer_info.clone(), 
            stream: stream_res.unwrap()
        };
        self._peers.push(new_peer);
        return Ok(());
    }

    
    // TODO: make error enum for this
    fn initiate_handshake(
        &mut self,
        peer_ind: usize,
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

        let stream = &mut self._peers[peer_ind].stream;
        let async_std_adapter: TokioCompatFix<&mut TokioTcpStream> = TokioCompatFix { 
            0: stream
        };

        let server_pk: PublicKey = self._peers[peer_ind].metadata.public_key.clone();

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
    is_handshaked: bool
}


fn get_peers_from_disk() -> Option<Vec<SSBPeer>> 
{ 

}

struct SSBTcpServer 
{
    _listener: TokioTcpListener,
    _client: SSBTcpClient,
}

impl SSBTcpServer 
{
    pub fn new() -> SSBTcpServer
    {
        let port: &str = ":3501";
        let public_addr: String = "0.0.0.0".to_owned() + port;

        let tcp_result: Result<TokioTcpListener, kuska_async_std::Error> = TokioTcpListener::bind(public_addr).await;
        if tcp_result.is_err() {
            return Err("Failed to bind TcpListener.".to_owned());
        }

        let peer_result: Option<Vec<SSBPeerInfo>> = get_peers_from_disk();
        let peers = vec![];
        if let Some(some_peers) = peer_result {
            peers = some_peers;
        }

        let client_result = SSBTcpClient::new(&peers);

        SSBTcpServer::handshake_peers(&mut client_result);
        return SSBTcpServer {
            _listener: tcp_result.unwrap(),
            _client: client_result,
        }
    }

    fn handshake_peers(client: &mut SSBTcpClient)
    {
        let peers: &Vec<SSBPeer> = client.get_peers();
        let p_enumerate = peers.iter().enumerate();

        for (ind, p) in p_enumerate {
            if !(p.metadata.is_handshaked) {
                let hs_result: Result<HandshakeComplete, String> = client.initiate_handshake(
                    ind, 
                    false
                );

                if hs_result.is_err() {
                    // see TODO above SSBTcpClient def
                } else {
                    p.metadata.is_handshaked = true;
                }
            }
        }
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
    let res_tuple = socket.recv_from(&mut buf).await?;
    return Ok((buf, res_tuple.1));
}
