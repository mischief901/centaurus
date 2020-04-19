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

use rustler;

use tokio::{
    sync::mpsc::unbounded_channel
};

use std::{
    path::PathBuf
};

type Result<T> = std::result::Result<T, Error>;

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

