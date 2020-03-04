mod error;
mod options;
mod net;
use error::{ ApplicationError };
use options::{ QuicOptions };
use net::Net;

#[macro_use]
extern crate rustler;
use rustler::{ Decoder, Encoder, Env, LocalPid, Term };
//use rustler::types::{ Binary };
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
    task::{ Context },
    path::PathBuf,
    sync::Arc,
};

atoms! {
    ok,
    error,
    none,
    bi,
    uni,
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
    ]
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
    socket: Option<QuicSocket>,
    socket_addr: Option<SocketAddr>,
    server_name: String,
    options: Vec<QuicOptions>,
    private_key: Option<PrivateKey>,
    certificates: Option<Certificates>,
}

#[derive(NifStruct)]
#[module="QuicStream"]
#[rustler(encode, decode)]
pub struct ElixirStream {
    stream_id: QuicStream,
    socket_id: QuicSocket,
    direction: Direction,
    options: Vec<QuicOptions>,
}

#[derive(NifUnitEnum)]
#[rustler(encode, decode)]
pub enum Direction {
    Bi,
    Uni,
}

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct QuicStream(LocalPid);

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct QuicSocket(LocalPid);

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: ElixirInterface, _timeout: u64) -> ElixirInterface {
    quic_socket
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(quic_socket: ElixirInterface, _timeout: u64) -> ElixirInterface {
    quic_socket
}

/// close(quic_socket, error_code)
#[rustler::nif]
fn close(quic_socket: ElixirInterface, _error_code: ApplicationError) -> ElixirInterface {
    quic_socket
}

/// close_stream(quic_stream, error_code)
#[rustler::nif]
fn close_stream(quic_stream: ElixirStream, _error_code: ApplicationError) -> ElixirStream {
    quic_stream
}

/// listen(quic_socket)
#[rustler::nif]
fn listen(quic_stream: ElixirInterface) -> ElixirInterface {
    quic_stream
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: ElixirStream, _direction: Direction) -> ElixirStream {
    quic_socket
}

/// read(quic_stream, timeout)
#[rustler::nif]
fn read(quic_stream: ElixirStream, _timeout: u64) -> ElixirStream {
    quic_stream
}

/// write(quic_stream, data)
#[rustler::nif]
fn write<'a>(quic_stream: ElixirStream, _data: &'a str) -> ElixirStream {
    quic_stream
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

impl Net for ElixirInterface {
    type E = rustler::Error;
    type S = ElixirStream;
    
    fn address(&self) -> Option<std::net::SocketAddr> {
        self.socket_addr
            .map(|SocketAddr(socket)| socket)
            .to_owned()
    }

    fn configure_client(&self) -> Result<EndpointBuilder, rustler::Error> {
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

    fn configure_server(&self) -> Result<ServerConfig, rustler::Error> {
        // TODO: This needs some research of what to do.
        let server_config = ServerConfig {
            transport: Arc::new(TransportConfig {
                stream_window_uni: 0,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut server_builder = ServerConfigBuilder::new(server_config);
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
        server_builder.certificate(cert, priv_key)
            .or(Err(rustler::Error::BadArg))?;
        let config = server_builder.build()
            .to_owned();
        Ok(config)
    }

    fn new_connection(&self, connection : NewConnection, ctx : &mut Context) {
        unimplemented!()
    }

    fn server_name(&self) -> &str {
        &self.server_name
    }

    fn new_peer_stream(&self) -> () {
        unimplemented!()
    }

    fn new_owned_stream(&self) -> Result<ElixirStream, rustler::Error> {
        unimplemented!()
    }
}


