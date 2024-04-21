// copy of code from kuska_ssb with following fixes:
// 1. TokioCompat has a public T
// 2. HandshakeComplete derives copy trait

use tokio::net::{
    tcp::{ReadHalf, WriteHalf},
    TcpStream,
};

use tokio::io::ReadBuf;

use futures::{
    io::{self, AsyncRead as Read, AsyncWrite as Write},
    task::{Context, Poll},
};
use std::pin::Pin;

use kuska_handshake::{HandshakeComplete, SharedSecret};
use kuska_sodiumoxide::crypto::{auth, scalarmult::curve25519};
use kuska_sodiumoxide::crypto::sign::ed25519;

#[derive(Clone)]
pub struct HandshakeCompleteFix 
{
    pub net_id: auth::Key,
    pub pk: ed25519::PublicKey,
    pub ephemeral_pk: curve25519::GroupElement,
    pub peer_pk: ed25519::PublicKey,
    pub peer_ephemeral_pk: curve25519::GroupElement,
    pub shared_secret: SharedSecret,
}

impl HandshakeCompleteFix 
{
    pub fn clone_org_to_fix(original: HandshakeComplete) -> HandshakeCompleteFix
    {
        return HandshakeCompleteFix {
            net_id: original.net_id,
            pk: original.pk,
            ephemeral_pk: original.ephemeral_pk,
            peer_pk: original.peer_pk,
            peer_ephemeral_pk: original.peer_ephemeral_pk,
            shared_secret: original.shared_secret,
        }
    }
}

pub struct TokioCompatFix<T>(pub T);

impl<T> TokioCompatFix<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub trait TokioCompatFixExt: tokio::io::AsyncRead + tokio::io::AsyncWrite + Sized {
    #[inline]
    fn wrap(self) -> TokioCompatFix<Self> {
        TokioCompatFix(self)
    }
}

pub trait TokioCompatFixExtRead: tokio::io::AsyncRead + Sized {
    #[inline]
    fn wrap(self) -> TokioCompatFix<Self> {
        TokioCompatFix(self)
    }
}

pub trait TokioCompatFixExtWrite: tokio::io::AsyncWrite + Sized {
    #[inline]
    fn wrap(self) -> TokioCompatFix<Self> {
        TokioCompatFix(self)
    }
}

impl<T: tokio::io::AsyncRead + Unpin> Read for TokioCompatFix<T> {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut read_buf = ReadBuf::new(buf);
        Pin::new(&mut self.0)
            .poll_read(cx, &mut read_buf)
            .map(|res| res.map(|()| read_buf.filled().len()))
    }
}

impl<T: tokio::io::AsyncWrite + Unpin> Write for TokioCompatFix<T> {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

impl TokioCompatFixExt for TcpStream {}
impl TokioCompatFixExtRead for ReadHalf<'_> {}
impl TokioCompatFixExtWrite for WriteHalf<'_> {}
impl TokioCompatFixExtRead for tokio::io::ReadHalf<TcpStream> {}
impl TokioCompatFixExtWrite for tokio::io::WriteHalf<TcpStream> {}