//! The Elixir entrypoint.
use super::types;
use super::types::{
    BeamSocket,
    BeamStream,
    Socket,
    SocketType,
    SocketAddr,
    Stream,
    StreamType,
};

use crate::conn;
use crate::error::{ ApplicationError, Error };

use std::convert::TryInto;

#[rustler::nif]
fn get_socket_config() -> Result<BeamSocket, Error> {
    let socket = types::test_socket();
    Ok(socket)
}

#[rustler::nif]
fn get_stream_config() -> Result<BeamStream, Error> {
    let stream = types::test_stream();
    Ok(stream)
}

/// listen(quic_socket)
#[rustler::nif]
fn listen(socket_config: BeamSocket, stream_config: BeamStream) -> Result<(), Error> {
    let socket = conn::Socket::new(SocketType::Server, socket_config.into(), stream_config.into())?;
    socket.listen()
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(socket_config: BeamSocket, stream_config: BeamStream, address: SocketAddr, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = conn::Socket::new(SocketType::Client, socket_config.into(), stream_config.into())?;
    let socket = socket.connect(*address, timeout)?;
    Ok(socket.into())
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Socket, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = quic_socket.accept(timeout)?
        .into();
    Ok(socket)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Socket, stream_type: StreamType) -> Result<Stream, Error> {
    let stream : Stream = match stream_type {
        StreamType::Uni => quic_socket.new_uni_stream()?.into(),
        StreamType::Bi => quic_socket.new_bi_stream()?.into(),
    };
    Ok(stream)
}

/// read(quic_stream, amount, timeout)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>, Error> {
    if let Ok(amount) = amount.try_into() {
        let buffer = Vec::with_capacity(amount);
        match quic_stream.read(buffer.clone(), timeout) {
            Ok(_length) => Ok(buffer),
            Err(error) => Err(error),
        }
    } else {
        Err(Error::Error)
    }
}

/// write(quic_stream, data)
#[rustler::nif]
fn write<'a>(quic_stream: Stream, data: &'a str) -> Result<(), Error> {
    let buffer = Vec::from(data);
    quic_stream.write(buffer)
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Socket, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
    quic_socket.close(error_code, reason)
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
    quic_stream.close_stream(error_code, reason)
}

