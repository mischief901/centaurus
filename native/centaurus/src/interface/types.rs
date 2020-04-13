//! The types used by Elixir and Quic.

use crate::config::{ ConnType };
use crate::conn;
use crate::error::Error;
use crate::options::{ QuicOptions };

use rustler::{ LocalPid, ResourceArc, NifStruct, NifTuple, NifUnitEnum };

use std::{
    ops::{ Deref },
    path::{ PathBuf },
    sync::{ Arc, RwLock },
};

//#[derive(NifTuple)]
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
#[module="QuicSocket"]
#[rustler(encode, decode)]
pub struct ElixirInterface {
    pub socket_pid: Option<QuicSocket>,
    pub socket_addr: Option<SocketAddr>,
    pub server_name: String,
    pub socket_owner: ConnectionOwner,
    pub options: Vec<QuicOptions>,
    pub private_key: Option<PrivateKey>,
    pub certificates: Option<Certificates>,
}

#[derive(NifStruct)]
#[module="QuicStream"]
#[rustler(encode, decode)]
pub struct ElixirStream {
    pub stream_pid: Option<QuicStream>,
    pub socket_pid: Option<QuicSocket>,
    pub stream_type: StreamType,
    pub stream_owner: ConnectionOwner,
    pub options: Vec<QuicOptions>,
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
pub struct SocketRef(pub Arc<RwLock<ElixirInterface>>);
#[derive(Clone)]
pub struct StreamRef(pub Arc<RwLock<ElixirStream>>);

// type aliases to make things easier to read.
type StreamConn = conn::Stream<StreamRef>;
type SocketConn = conn::Socket<SocketRef, StreamRef>;

/// The Socket and Stream newtype structs are used to create a Rust representation
/// of the connection that Elixir can use to identify the connection in the future.
#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct Socket(pub ResourceArc<SocketInterior>);
pub struct SocketInterior(pub SocketConn);

#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct Stream(pub ResourceArc<StreamInterior>);
pub struct StreamInterior(pub StreamConn);

impl Deref for Socket {
    type Target = SocketInterior;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SocketInterior {
    type Target = SocketConn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SocketRef {
    type Target = RwLock<ElixirInterface>;

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
    type Target = StreamConn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for StreamRef {
    type Target = RwLock<ElixirStream>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<SocketRef> for ElixirInterface {
    fn into(self) -> SocketRef {
        SocketRef(Arc::new(RwLock::new(self)))
    }
}

impl Into<StreamRef> for ElixirStream {
    fn into(self) -> StreamRef {
        StreamRef(Arc::new(RwLock::new(self)))
    }
}

impl Socket {
    pub fn new(interface: ElixirInterface, conn_type: ConnType, stream_config: Option<StreamRef>) -> Result<Self, Error> {
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

impl From<SocketConn> for Socket {
    fn from(socket_int : SocketConn) -> Self {
        Socket(ResourceArc::new(socket_int.into()))
    }
}

impl From<SocketConn> for SocketInterior {
    fn from(socket_int : SocketConn) -> Self {
        SocketInterior(socket_int)
    }
}

impl From<StreamConn> for Stream {
    fn from(stream_int : StreamConn) -> Self {
        Stream(ResourceArc::new(stream_int.into()))
    }
}

impl From<StreamConn> for StreamInterior {
    fn from(stream_int : StreamConn) -> Self {
        StreamInterior(stream_int)
    }
}

impl Default for StreamRef {
    fn default() -> Self {
        ElixirStream::default().into()
    }
}

impl Default for ElixirStream {
    fn default() -> Self {
        unimplemented!();
    }
}

