mod ssb_id;
mod tokio_compat_fix;

use std::collections::HashMap;

use kuska_ssb::{crypto::ed25519::{PublicKey, SecretKey}, keystore::OwnedIdentity};
use kuska_sodiumoxide::crypto::{auth, sign::ed25519};

use kuska_handshake::{async_std as kuska_async_std, HandshakeComplete};

use async_std;

// handshake_client
use tokio::{self, 
    net::TcpListener as TokioTcpListener, 
    net::TcpStream as TokioTcpStream
}; // 1.37.0

use tokio_compat_fix::TokioCompatFix;

// struct HandshakeHandler 
// {
// }

// impl HandshakeHandler 
// {
    
// }


static _TCP_ENDPOINTS: HashMap<&str, &str> = HashMap::from([
    ("root", "/"),
]);

struct SSBTcpClient 
{
    _streams: TokioTcpStream
}

impl SSBTcpClient
{
    fn new() -> SSBTcpClient {  }
}

// list of peers will be used later when requests need to go to specific peers
struct SSBPeer 
{
    addr: String
}

struct SSBTcpServer 
{
    _listener: TokioTcpListener,
    //_handshaker: HandshakeHandler,
    _client: SSBTcpClient,
    _peers: Vec<SSBPeer>
}

impl SSBTcpServer 
{
    pub fn new() -> SSBTcpServer
    {
        let port: &str = ":3501";
        let public_addr: String = "0.0.0.0".to_owned() + port;

        let result: Result<TokioTcpListener, kuska_async_std::Error> = TokioTcpListener::bind(public_addr).await;

    }

    // TODO: make error enum for this
    fn initiate_handshake(
        stream: &mut TokioTcpStream,
        net_id: auth::Key,
        id: OwnedIdentity,
        server_pk: ed25519::PublicKey,
    ) -> Result<HandshakeComplete, String> 
    {
        let async_std_adapter: TokioCompatFix<&mut TokioTcpStream> = TokioCompatFix { 0: stream };

        let client_sk: SecretKey = id.sk;
        let client_pk: PublicKey = id.pk;

        // returns custom Result type from kuska_ssb
        let handshake_res = kuska_async_std::handshake_client(
            &mut async_std_adapter, 
            net_id, 
            client_pk, 
            client_sk, 
            server_pk
        ).await;

        if handshake_res.is_err() {
            return Err("Failed to perform handshake.".to_owned());
        }

        return handshake_res;
    }
}


enum SSBDiscoveryMethod
{
    // only invitecode will be implemented for now
    LANBroadcast,
    InviteCode,
    BluetoothBroadcast
}

