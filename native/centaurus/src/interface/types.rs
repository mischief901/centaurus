//! The types used by the Beam and Quic.
use crate::conn;
use crate::options::{ QuicOptions };

use rustler::{ LocalPid, ResourceArc, NifStruct, NifTuple, NifUnitEnum };

use tokio::{
    sync::{ RwLock },
};

use std::{
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
pub struct BeamSocket {
    pub socket_pid: Option<QuicSocket>,
    pub bind_address: Option<SocketAddr>,
    pub server_name: String,
    pub options: Vec<QuicOptions>,
    pub private_key: Option<PrivateKey>,
    pub certificates: Option<Certificates>,
}

pub fn test_socket() -> BeamSocket {
    BeamSocket {
        socket_pid: None,
        bind_address: Some(SocketAddr("127.0.0.1:0".parse().unwrap())),
        server_name: "localhost".to_string(),
        options: vec!(),
        private_key: Some(PrivateKey(PathBuf::from("/"))),
        certificates: Some(Certificates(PathBuf::from("/")))
    }
}

#[derive(NifStruct)]
#[module="Centaurus.Types.StreamConfig"]
#[rustler(encode, decode)]
pub struct BeamStream {
    pub stream_pid: Option<QuicStream>,
    pub socket_pid: Option<QuicSocket>,
    pub stream_type: StreamType,
    pub options: Vec<QuicOptions>,
}

pub fn test_stream() -> BeamStream {
    BeamStream {
        stream_pid: None,
        socket_pid: None,
        stream_type: StreamType::Bi,
        options: vec!()
    }
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
pub enum SocketType {
    Server,
    Client,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
pub enum ConnectionOwner {
    Peer,
    Host,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
pub enum StreamType {
    Bi,
    Uni,
}

#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct QuicStream(pub LocalPid);

#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct QuicSocket(pub LocalPid);

/// The SocketRef and StreamRef newtype structs are used to wrap the Socket and Stream interfaces
/// in a RwLock. A RwLock is used instead of a Mutex because there should be little change (writes)
/// performed on these data structures. They contain data used to setup the connection or the
/// information necessary to send received messages or errors to the owners (PIDs).
#[derive(Clone)]
pub struct SocketRef(pub Arc<BeamSocket>);
#[derive(Clone)]
pub struct StreamRef(pub Arc<BeamStream>);

/// The Socket and Stream newtype structs are used to create a Rust representation
/// of the connection that Elixir can use to identify the connection in the future.
#[derive(NifTuple)]
#[rustler(encode, decode)]
#[derive(Clone)]
pub struct Socket(pub ResourceArc<SocketInterior>);
pub struct SocketInterior(conn::Socket);

#[derive(NifTuple)]
#[rustler(encode, decode)]
#[derive(Clone)]
pub struct Stream(pub ResourceArc<StreamInterior>);
pub struct StreamInterior(conn::Stream);

impl Deref for Socket {
    type Target = SocketInterior;
    
    fn deref(&self) -> &Self::Target {
        &self.0
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
        &self.0
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

/*
impl Socket {
    pub fn new(interface: BeamSocket, conn_type: ConnType, stream_config: Option<StreamRef>) -> Result<Self, Error> {
        conn::Socket::new(interface.into(), conn_type, stream_config)
            .map(|conn| SocketInterior::new(conn))
            .map(|conn| Socket(ResourceArc::new(conn)))
    }
}

impl SocketInterior {
    pub fn new(socket: SocketConn) -> Self {
        SocketInterior(socket)
    }
}

impl Stream {
    pub fn new(stream_int : Result<StreamConn, Error>) -> Result<Self, Error> {
        stream_int
            .map(|conn| StreamInterior::new(conn))
            .map(|stream| Stream(ResourceArc::new(stream)))
    }
}

impl StreamInterior {
    pub fn new(stream: StreamConn) -> Self {
        StreamInterior(stream)
    }
}
*/ 

impl From<conn::Socket> for Socket {
    fn from(socket_int : conn::Socket) -> Self {
        Socket(ResourceArc::new(socket_int.into()))
    }
}

impl From<conn::Socket> for SocketInterior {
    fn from(socket_int : conn::Socket) -> Self {
        SocketInterior(socket_int)
    }
}

impl From<conn::Stream> for Stream {
    fn from(stream_int : conn::Stream) -> Self {
        Stream(ResourceArc::new(stream_int.into()))
    }
}

impl From<conn::Stream> for StreamInterior {
    fn from(stream_int : conn::Stream) -> Self {
        StreamInterior(stream_int)
    }
}

