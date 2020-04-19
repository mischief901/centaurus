//! The types used by the Beam and Quic.
use crate::conn;
use crate::options::{ QuicOptions };

use rustler::{
    LocalPid,
    ResourceArc,
    NifStruct,
    NifTuple,
    NifUnitEnum,
    NifUntaggedEnum,
};

use std::{
    fmt,
    ops::{ Deref },
    path::{ PathBuf },
    sync::{ Arc },
};

/// "127.0.0.1:8080" on the Elixir side.
#[derive(Debug, Copy, Clone)]
pub struct SocketAddr(pub std::net::SocketAddr);

impl Deref for SocketAddr {
    type Target = std::net::SocketAddr;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct PrivateKey(pub PathBuf);

#[derive(Debug)]
pub struct Certificates(pub PathBuf);

#[derive(NifStruct)]
#[module="Centaurus.Types.SocketConfig"]
#[rustler(encode, decode)]
#[derive(Debug)]
pub struct BeamSocket {
    pub socket_pid: Option<QuicSocket>,
    pub bind_address: Option<SocketAddr>,
    pub server_name: String,
    pub options: QuicOptions,
    pub private_key: Option<PrivateKey>,
    pub certificates: Option<Certificates>,
}

#[derive(NifStruct)]
#[module="Centaurus.Types.StreamConfig"]
#[rustler(encode, decode)]
#[derive(Debug)]
pub struct BeamStream {
    pub stream_pid: Option<QuicStream>,
    pub stream_type: StreamType,
    pub options: QuicOptions,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
#[derive(Debug)]
pub enum SocketType {
    Server,
    Client,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
#[derive(Debug)]
pub enum ConnectionOwner {
    Peer,
    Host,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
#[derive(Debug)]
pub enum StreamType {
    Bi,
    Uni,
}

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error(error)
    }
}

impl Deref for Error {
    type Target = anyhow::Error;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(NifUntaggedEnum)]
#[rustler(encode, decode)]
pub enum QuicStream {
    Pid(LocalPid)
}

impl fmt::Debug for QuicStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream Pid.")
    }
}

#[derive(NifUntaggedEnum)]
#[rustler(encode, decode)]
pub enum QuicSocket{
    Pid(LocalPid)
}

impl fmt::Debug for QuicSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket Pid.")
    }
}

/// The SocketRef and StreamRef newtype structs are used to wrap the Socket and Stream interfaces
/// in a RwLock. A RwLock is used instead of a Mutex because there should be little change (writes)
/// performed on these data structures. They contain data used to setup the connection or the
/// information necessary to send received messages or errors to the owners (PIDs).
#[derive(Clone, Debug)]
pub struct SocketRef(pub Arc<BeamSocket>);
#[derive(Clone, Debug)]
pub struct StreamRef(pub Arc<BeamStream>);

/// The Socket and Stream newtype structs are used to create a Rust representation
/// of the connection that Elixir can use to identify the connection in the future.
#[derive(NifUntaggedEnum)]
#[rustler(encode, decode)]
#[derive(Clone)]
pub enum Socket { Socket(ResourceArc::<SocketInterior>) }
pub struct SocketInterior(conn::Socket);

#[derive(NifUntaggedEnum)]
#[rustler(encode, decode)]
#[derive(Clone)]
pub enum Stream { Stream(ResourceArc::<StreamInterior>) }
pub struct StreamInterior(conn::Stream);

impl Deref for Socket {
    type Target = SocketInterior;
    
    fn deref(&self) -> &Self::Target {
        let Socket::Socket(socket) = self;
        &socket
    }
}

impl Deref for SocketInterior {
    type Target = conn::Socket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SocketRef {
    type Target = BeamSocket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Stream {
    type Target = StreamInterior;
    
    fn deref(&self) -> &Self::Target {
        let Stream::Stream(stream) = self;
        &stream
    }
}

impl Deref for StreamInterior {
    type Target = conn::Stream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for StreamRef {
    type Target = BeamStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<SocketRef> for BeamSocket {
    fn into(self) -> SocketRef {
        SocketRef(Arc::new(self))
    }
}

impl Into<StreamRef> for BeamStream {
    fn into(self) -> StreamRef {
        StreamRef(Arc::new(self))
    }
}

impl From<conn::Socket> for Socket {
    fn from(socket_int : conn::Socket) -> Self {
        Socket::Socket(ResourceArc::new(socket_int.into()))
    }
}

impl From<conn::Socket> for SocketInterior {
    fn from(socket_int : conn::Socket) -> Self {
        SocketInterior(socket_int)
    }
}

impl From<conn::Stream> for Stream {
    fn from(stream_int : conn::Stream) -> Self {
        Stream::Stream(ResourceArc::new(stream_int.into()))
    }
}

impl From<conn::Stream> for StreamInterior {
    fn from(stream_int : conn::Stream) -> Self {
        StreamInterior(stream_int)
    }
}

