/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
use crate::error::{ Error, ApplicationError };
use crate::options::{ QuicOptions };
use crate::config::Config;
use crate::conn;
use crate::runtime::{ Handle, RunSocket, RunStream, Runtime };

use rustler::{ Decoder, Encoder, Env, LocalPid, ResourceArc, Term };
use rustler::types::{ tuple };
use rustler_codegen::{ NifStruct, NifTuple, NifUnitEnum };

use quinn::{
    Certificate,
    CertificateChain,
    ClientConfigBuilder,
    Endpoint,
    EndpointBuilder,
    NewConnection,
    ServerConfig,
    ServerConfigBuilder,
    TransportConfig
};

use std::{
    fs,
    path::PathBuf,
    sync::Arc,
};

atoms! {
    ok,
    error,
    none,
    bi,
    uni,
    peer,
    host,
}

init!(
    "Elixir.Centaurus",
    [
        accept,
        connect,
        close,
        close_stream,
        listen,
        open_stream,
        read,
        write,
    ],
    load = setup_runtime
);

//#[derive(NifTuple)]
/// "127.0.0.1:8080" on the Elixir side.
#[derive(Debug, Copy, Clone)]
struct SocketAddr(std::net::SocketAddr);

#[derive(Debug)]
struct PrivateKey(PathBuf);

#[derive(Debug)]
struct Certificates(PathBuf);

#[derive(NifStruct)]
#[module="QuicSocket"]
#[rustler(encode, decode)]
pub struct ElixirInterface {
    socket_pid: Option<QuicSocket>,
    socket_addr: Option<SocketAddr>,
    server_name: String,
    socket_owner: ConnectionOwner,
    options: Vec<QuicOptions>,
    private_key: Option<PrivateKey>,
    certificates: Option<Certificates>,
}

#[derive(NifStruct)]
#[module="QuicStream"]
#[rustler(encode, decode)]
pub struct ElixirStream {
    stream_pid: Option<QuicStream>,
    socket_pid: Option<QuicSocket>,
    stream_type: StreamType,
    stream_owner: ConnectionOwner,
    options: Vec<QuicOptions>,
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
struct QuicStream(LocalPid);

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct QuicSocket(LocalPid);

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct SocketHandler(ResourceArc<Handle>);

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct StreamHandler(ResourceArc<Handle>);

type Connection = conn::Connection<ElixirInterface, SocketHandler>;

type Stream = conn::Stream<ElixirStream, StreamHandler>;

fn setup_runtime(env: Env, _: Term) -> bool {
    // The Tokio runtime.
    resource!(Runtime, env);
    // Handles to jobs in the Tokio runtime.
    resource!(Handle, env);
    // A set up quic connection
    resource!(Connection, env);
    // An open stream connection
    resource!(Stream, env);

    // Initiate the runtime.
    Runtime::new();
    true
}

/// listen(quic_socket)
#[rustler::nif]
fn listen(quic_socket: ElixirInterface) -> Result<Connection, Error> {
    Connection::new_server(quic_socket)
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(quic_socket: ElixirInterface, timeout: u64) -> Result<Connection, Error> {
    Connection::new_client(quic_socket)
        .handler()
        .connect(timeout)
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Connection, timeout: u64) -> Result<Connection, Error> {
    quic_socket.handler()
        .accept(timeout)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Connection, stream_type: StreamType) -> Result<Stream, Error> {
    match stream_type {
        StreamType::Uni => quic_socket.handler().new_uni_stream(),
        StreamType::Bi => quic_socket.handler().new_bi_stream(),
    }
}

/// read(quic_stream, amount)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>, Error> {
    quic_stream.handler().read(amount, timeout)
}

/// write(quic_stream, data)
#[rustler::nif]
fn write<'a>(quic_stream: Stream, data: &'a str) -> Result<(), Error> {
    quic_stream.handler().write(data)
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Connection, error_code: ApplicationError, reason: Vec<u8>) {
    quic_socket.handler().close(error_code, reason)
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Vec<u8>) {
    quic_stream.handler().close_stream(error_code, reason)
}


impl<'a> Decoder<'a> for SocketAddr {
    fn decode(term : Term<'a>) -> Result<SocketAddr, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let sock_addr : std::net::SocketAddr = raw
            .parse()
            .or(Err(rustler::Error::BadArg))?;
        Ok(SocketAddr(sock_addr))
    }
}

impl<'a> Encoder for SocketAddr {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let SocketAddr(sock_addr) = self;
        sock_addr.to_string().encode(env)
    }
}

impl<'a> Decoder<'a> for Certificates {
    fn decode(term : Term<'a>) -> Result<Self, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let mut path = PathBuf::new();
        path.push(raw);
        Ok(Certificates(path))
    }
}

impl<'a> Encoder for Certificates {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let Certificates(path) = self;
        path.to_str().encode(env)
    }
}

impl<'a> Decoder<'a> for PrivateKey {
    fn decode(term : Term<'a>) -> Result<Self, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let mut path = PathBuf::new();
        path.push(raw);
        Ok(PrivateKey(path))
    }
}

impl<'a> Encoder for PrivateKey {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let PrivateKey(path) = self;
        path.to_str().encode(env)
    }
}

impl<'a> Decoder<'a> for Connection {
    fn decode(term: Term<'a>) -> Result<Self, rustler::Error> {
        match tuple::get_tuple(term) {
            Err(error) => Err(error),
            Ok(tuple) => {
                let meta : ElixirInterface = Decoder::decode(tuple[0])?;
                let conn : SocketHandler = Decoder::decode(tuple[1])?;
                Ok(Connection{
                    meta,
                    conn
                })
            }
        }
    }
}

impl<'a> Encoder for Connection {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        make_tuple(env, vec![self.meta.encode(env), self.conn.encode(env)])
    }
}

impl<'a> Decoder<'a> for Stream {
    fn decode(term: Term<'a>) -> Result<Self, rustler::Error> {
        match tuple::get_tuple(term) {
            Err(error) => Err(error),
            Ok(tuple) => {
                let meta : ElixirStream = Decoder::decode(tuple[0])?;
                let conn : StreamHandler = Decoder::decode(tuple[1])?;
                Ok(Stream{
                    meta,
                    conn
                })
            }
        }
    }
}

impl<'a> Encoder for Stream {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        make_tuple(env, vec![self.meta.encode(env), self.conn.encode(env)])
    }
}

impl Certificates {
    fn as_chain(&self) -> Option<CertificateChain> {
        let Certificates(cert_path) = self;
        let raw_certs = fs::read(cert_path).ok()?;
        if cert_path.extension().map_or(false, |x| x == "der") {
            Some(CertificateChain::from_certs(Certificate::from_der(&raw_certs)))
        } else {
            CertificateChain::from_pem(&raw_certs).ok()
        }
    }

    fn as_cert(&self) -> Option<Certificate> {
        let Certificates(cert_path) = self;
        quinn::Certificate::from_der(&fs::read(cert_path).ok()?)
            .ok()
    }
}

impl PrivateKey {   
    fn as_key(&self) -> Option<quinn::PrivateKey> {
        let PrivateKey(path) = self;
        let raw_key = fs::read(path).ok()?;
        if path.extension().map_or(false, |x| x == "der") {
            quinn::PrivateKey::from_der(&raw_key).ok()
        } else {
            quinn::PrivateKey::from_pem(&raw_key).ok()
        }
    }
}

impl Config for ElixirInterface {
    type E = rustler::Error;
    
    fn address(&self) -> Result<std::net::SocketAddr, Self::E> {
        self.socket_addr
            .map(|SocketAddr(socket)| socket)
            .to_owned()
            .ok_or(rustler::Error::BadArg)
    }

    fn configure_client(&self) -> Result<EndpointBuilder, Self::E> {
        let mut client_builder = ClientConfigBuilder::default();
        let cert : Certificate = self.certificates
            .as_ref()
            .ok_or(rustler::Error::BadArg)?
            .as_cert()
            .ok_or(rustler::Error::BadArg)?;
        client_builder.add_certificate_authority(cert).or(Err(rustler::Error::BadArg))?;
        let mut endpoint = Endpoint::builder();
        Ok(endpoint.default_client_config(client_builder.build()).to_owned())
    }

    fn configure_server(&self) -> Result<EndpointBuilder, Self::E> {
        let server_config = ServerConfig {
            transport: Arc::new(TransportConfig {
                stream_window_uni: 0,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut server_conf_builder = ServerConfigBuilder::new(server_config);
        let cert : CertificateChain = self.certificates
            .as_ref()
            .ok_or(rustler::Error::BadArg)?
            .as_chain()
            .ok_or(rustler::Error::BadArg)?;
        let priv_key : quinn::PrivateKey = self.private_key
            .as_ref()
            .ok_or(rustler::Error::BadArg)?
            .as_key()
            .ok_or(rustler::Error::BadArg)?;
        server_conf_builder.certificate(cert, priv_key)
            .or(Err(rustler::Error::BadArg))?;
        let mut server_builder = Endpoint::builder();
        Ok(server_builder.listen(server_conf_builder.build()).to_owned())
    }

    fn server_name(&self) -> &str {
        &self.server_name
    }
}


//struct SocketHandler(ResourceArc<Handle>);

impl RunSocket for SocketHandler {
    type Socket = ElixirInterface;
    type Stream = ElixirStream;

    fn accept(&self, timeout: Option<u64>) -> Self::Socket {
        unimplemented!();
    }

    fn new_uni_stream(&self) -> {
        unimplemented!();
    }

    fn new_bi_stream(&self) -> {
        unimplemented!();
    }

    fn new_peer_uni_stream(&self) -> {
        unimplemented!();
    }

    fn new_peer_bi_stream(&self) -> {
        unimplemented!();
    }

    fn close(&self, error_code: ApplicationError, reason: Vec<u8>) {
        unimplemented!();
    }
}


impl RunStream for StreamHandler {
    type Stream = ElixirStream;

    fn read(&self, buffer: &[u8], timeout: Option<u64>) -> Result<Vec<u8>, Error> {
        unimplemented!();
    }

    fn write(&self, buffer: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>) {
        unimplemented!();
    }   
}
