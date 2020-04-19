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

use tokio::{
    sync::mpsc::unbounded_channel
};

use std::{
    path::PathBuf
};

type Result<T> = std::result::Result<T, Error>;

#[rustler::nif]
fn encode_socket_config(_socket: BeamSocket) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn encode_stream_config(_stream: BeamStream) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_socket_config() -> Result<BeamSocket> {
    Ok(BeamSocket {
        socket_pid: None,
        bind_address: Some(SocketAddr("127.0.0.1:0".parse().unwrap())),
        server_name: "localhost".to_string(),
        options: vec!(),
        private_key: Some(PrivateKey(PathBuf::from("/"))),
        certificates: Some(Certificates(PathBuf::from("/")))
    })
}

#[rustler::nif]
fn decode_stream_config() -> Result<BeamStream> {
    Ok(BeamStream {
        stream_pid: None,
        stream_type: StreamType::Bi,
        options: vec!()
    })
}

#[rustler::nif]
fn encode_socket(_socket: Socket) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn encode_stream(_stream: Stream) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_socket() -> Result<Socket> {
    let (sender, _receiver) = unbounded_channel();
    Ok(Socket::from(crate::conn::Socket(sender)))
}

#[rustler::nif]
fn decode_stream() -> Result<Stream> {
    let (sender, _receiver) = unbounded_channel();
    Ok(Stream::from(crate::conn::Stream(sender)))
}

#[rustler::nif]
fn encode_application_error(_error: ApplicationError) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_application_error() -> Result<ApplicationError> {
    Ok(ApplicationError::Error(10))
}

#[rustler::nif]
fn encode_certificates(_certs: Certificates) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_certificates() -> Result<Certificates> {
    Ok(Certificates(PathBuf::from("/certificate")))
}

#[rustler::nif]
fn encode_private_key(_key: PrivateKey) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_private_key() -> Result<PrivateKey> {
    Ok(PrivateKey(PathBuf::from("/private_key")))
}

#[rustler::nif]
fn encode_socket_addr(_socket: SocketAddr) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_socket_addr() -> Result<SocketAddr> {
    Ok(SocketAddr("127.0.0.1:0".parse().unwrap()))
}

#[rustler::nif]
fn encode_stream_type(_stream_type: StreamType) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_stream_type() -> Result<StreamType> {
    Ok(StreamType::Bi)
}

#[rustler::nif]
fn encode_socket_type(_socket_type: SocketType) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_socket_type() -> Result<SocketType> {
    Ok(SocketType::Server)
}

#[rustler::nif]
fn encode_quic_opts(_opts: QuicOptions) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn decode_quic_opts() -> Result<Vec<QuicOptions>> {
    let option = QuicOptions::Timeout(5_000);
    Ok(vec!(option))
}

#[rustler::nif]
fn encode_quic_socket(_pid: Vec<QuicSocket>) -> Result<()> {
    Ok(())
}

#[rustler::nif]
fn encode_quic_stream(_pid: QuicStream) -> Result<()> {
    Ok(())
}

