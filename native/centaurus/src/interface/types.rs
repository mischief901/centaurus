//! The types used by Elixir and Quic.

use crate::config::Config;
use crate::conn;
use crate::error::Error;
use crate::options::{ QuicOptions };
use crate::runtime::{ Handle, Runtime };

use rustler::{ LocalPid, ResourceArc };
use rustler_codegen::{ NifStruct, NifTuple, NifUnitEnum };

use std::{
    path::PathBuf,
    sync::{ Mutex, RwLock },
};

//#[derive(NifTuple)]
/// "127.0.0.1:8080" on the Elixir side.
#[derive(Debug, Copy, Clone)]
pub struct SocketAddr(pub std::net::SocketAddr);

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

pub struct SocketHandler(pub Mutex<Handle>);
pub struct StreamHandler(pub Mutex<Handle>);

/// The SocketRef and StreamRef newtype structs are used to wrap the Socket and Stream interfaces
/// in a RwLock. A RwLock is used instead of a Mutex because there should be little change (writes)
/// performed on these data structures. They contain data used to setup the connection or the
/// information necessary to send received messages or errors to the owners (PIDs).
pub struct SocketRef(pub RwLock<ElixirInterface>);
pub struct StreamRef(pub RwLock<ElixirStream>);

/// The Connection and Stream newtype structs are used to create a Rust representation
/// of the connection that Elixir can use to identify the connection in the future.
#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct Connection(pub ResourceArc<conn::Connection<SocketRef, SocketHandler>>);

#[derive(NifTuple)]
#[rustler(encode, decode)]
pub struct Stream(pub ResourceArc<conn::Stream<StreamRef, StreamHandler>>);


impl Into<SocketRef> for ElixirInterface {
    fn into(self) -> SocketRef {
        SocketRef(RwLock::new(self))
    }
}

impl Into<StreamRef> for ElixirStream {
    fn into(self) -> StreamRef {
        StreamRef(RwLock::new(self))
    }
}

impl Connection {
    pub fn new_server(interface: ElixirInterface) -> Result<Self, Error> {
        conn::Connection::new_server(interface.into())
            .map(|conn| Connection(ResourceArc::new(conn)))
    }

    pub fn new_client(interface: ElixirInterface) -> Result<Self, Error> {
        conn::Connection::new_client(interface.into())
            .map(|conn| Connection(ResourceArc::new(conn)))
    }
    
    pub fn inner(&self) -> &conn::Connection<SocketRef, SocketHandler> {
        let Connection(conn) = self;
        &conn
    }
}

impl Stream {
    pub fn new(stream: ElixirStream) -> Result<Self, Error> {
        unimplemented!();
    }
    
    pub fn inner(&self) -> &conn::Stream<StreamRef, StreamHandler> {
        let Stream(stream) = self;
        &stream
    }
}


