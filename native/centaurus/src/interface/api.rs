//! The Elixir entrypoint.

use super::types::{
    BeamSocket,
    BeamStream,
    Socket,
    SocketAddr,
    Stream,
};

use crate::config::{ SocketType, StreamType };
use crate::error::{ ApplicationError, Error };

use std::convert::TryInto;

/// listen(quic_socket)
#[rustler::nif]
fn listen(socket_config: BeamSocket, stream_config: BeamStream, address: SocketAddr) -> Result<Socket, Error> {
    let socket = Socket::open_socket(SocketType::Server, socket_config, stream_config);
    socket.listen(address)
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(socket_config: BeamSocket, stream_config: BeamStream, address: SocketAddr, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = Socket::open_socket(SocketType::Client, socket_config, stream_config)?;
    (**socket).connect(*address, timeout)?;
    Ok(socket)
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Socket, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = (**quic_socket).accept(timeout)?
        .into();
    Ok(socket)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Socket, stream_type: StreamType) -> Result<Stream, Error> {
    let stream : Stream = match stream_type {
        StreamType::Uni => (**quic_socket).new_uni_stream()?.into(),
        StreamType::Bi => (**quic_socket).new_bi_stream()?.into(),
    };
    Ok(stream)
}

/// read(quic_stream, amount, timeout)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>, Error> {
    if let Ok(amount) = amount.try_into() {
        let buffer = Vec::with_capacity(amount);
        match (**quic_stream).read(buffer, timeout) {
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
    (**quic_stream).write(&buffer)
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Socket, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
    (**quic_socket).close(error_code, reason);
    Ok(())
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
    (**quic_stream).close_stream(error_code, reason);
    Ok(())
}

