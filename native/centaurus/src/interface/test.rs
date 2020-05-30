/// A few test functions to debug the api.
use super::types::{
    BeamSocket,
    BeamStream,
    Certificates,
    Error,
    PrivateKey,
    QuicSocket,
    QuicStream,
    Socket,
    SocketAddr,
    SocketType,
    StreamType,
    Stream
};

use crate::error::{ ApplicationError };
use crate::options::{ QuicOptions };

use anyhow::{ Context };

use rustler;
use rustler::{ Decoder, Encoder, Env, Term };

use tokio::{
    sync::mpsc::unbounded_channel
};

use std::{
    io::{ Write },
    fs::{ File },
    ops::{ Deref },
    path::PathBuf
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Directory(pub PathBuf);

impl Deref for Directory {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/*
impl DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut Self::Target {

    }
}*/

impl<'a> Decoder<'a> for Directory {
    fn decode(term : Term<'a>) -> std::result::Result<Self, rustler::Error> {
        let raw : &str = Decoder::decode(term)
            .or(Err(rustler::Error::Term(Box::new("Invalid Directory"))))?;
        let mut path = PathBuf::new();
        path.push(raw);
        Ok(Directory(path))
    }
}

impl<'a> Encoder for Directory {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let Directory(path) = self;
        path.to_str().encode(env)
    }
}

#[rustler::nif]
fn create_cert_and_key(path: Directory, name: Option<String>) -> Result<()> {
    let name = name.or(Some("localhost".to_string())).unwrap();
    let cert = rcgen::generate_simple_self_signed(vec![name]).unwrap();

    let cert_pem = cert.serialize_pem()
        .context("Error Serializing Cert.")?;
    let cert_der = cert.serialize_der()
        .context("Error Serializing Cert.")?;
    let priv_key_pem = cert.serialize_private_key_pem();
    let priv_key_der = cert.serialize_private_key_der();

    let cert_pem_name: PathBuf = [path.0.clone(), "cert.pem".into()].iter().collect();
    let cert_der_name: PathBuf = [path.0.clone(), "cert.der".into()].iter().collect();
    let priv_pem_name: PathBuf = [path.0.clone(), "key.pem".into()].iter().collect();
    let priv_der_name: PathBuf = [path.0.clone(), "key.der".into()].iter().collect();
    
    let mut cert_file_pem = File::create(cert_pem_name)
        .context("Error creating Cert File.")?;
    let mut cert_file_der = File::create(cert_der_name)
        .context("Error creating Cert File.")?;
    let mut priv_file_pem = File::create(priv_pem_name)
        .context("Error creating Private Key File.")?;
    let mut priv_file_der = File::create(priv_der_name)
        .context("Error creating Private Key File.")?;

    cert_file_pem.write_all(cert_pem.as_bytes()).context("Error Writing Certificate Pem")?;
    cert_file_der.write_all(&cert_der).context("Error Writing Certificate Der")?;
    priv_file_pem.write_all(priv_key_pem.as_bytes()).context("Error Writing Private Key Pem")?;
    priv_file_der.write_all(&priv_key_der).context("Error Writing Private Key Der")?;
    Ok(())
}

#[rustler::nif]
fn test_socket_config(socket: BeamSocket) -> Result<BeamSocket> {
    Ok(BeamSocket {
        socket_pid: socket.socket_pid,
        bind_address: Some(SocketAddr("127.0.0.1:0".parse().unwrap())),
        server_name: "localhost".to_string(),
        options: QuicOptions { timeout: None },
        private_key: Some(PrivateKey(PathBuf::from("/"))),
        certificates: Some(Certificates(PathBuf::from("/")))
    })
}

#[rustler::nif]
fn test_stream_config(stream: BeamStream) -> Result<BeamStream> {
    Ok(BeamStream {
        stream_pid: stream.stream_pid,
        stream_type: StreamType::Bi,
        options: QuicOptions { timeout: None }
    })
}

#[rustler::nif]
fn test_socket(socket: Socket) -> Result<Socket> {
    Ok(socket)
}

#[rustler::nif]
fn test_stream(stream: Stream) -> Result<Stream> {
    Ok(stream)
}

#[rustler::nif]
fn get_socket() -> Result<Socket> {
    let (sender, _receiver) = unbounded_channel();
    Ok(Socket::from(crate::conn::Socket(sender)))
}

#[rustler::nif]
fn get_stream() -> Result<Stream> {
    let (sender, _receiver) = unbounded_channel();
    Ok(Stream::from(crate::conn::Stream(sender)))
}

#[rustler::nif]
fn test_application_error(error: ApplicationError) -> Result<ApplicationError> {
    Ok(error)
}

#[rustler::nif]
fn test_certificates(certs: Certificates) -> Result<Certificates> {
    Ok(certs)
}

#[rustler::nif]
fn test_private_key(key: PrivateKey) -> Result<PrivateKey> {
    Ok(key)
}

#[rustler::nif]
fn test_socket_addr(socket: SocketAddr) -> Result<SocketAddr> {
    Ok(socket)
}

#[rustler::nif]
fn test_stream_type(stream_type: StreamType) -> Result<StreamType> {
    Ok(stream_type)
}

#[rustler::nif]
fn test_socket_type(socket_type: SocketType) -> Result<SocketType> {
    Ok(socket_type)
}

#[rustler::nif]
fn test_quic_opts(opts: QuicOptions) -> Result<QuicOptions> {
    Ok(opts)
}

#[rustler::nif]
fn test_quic_socket(pid: QuicSocket) -> Result<QuicSocket> {
    Ok(pid)
}

#[rustler::nif]
fn test_quic_stream(pid: QuicStream) -> Result<QuicStream> {
    Ok(pid)
}

