//! The Elixir entrypoint.

use super::types::{
    ElixirInterface,
    //    ElixirStream,
    Socket,
    SocketAddr,
    Stream,
    StreamType,
};

use crate::config::{ ConnType };
use crate::error::{ ApplicationError, Error };
use crate::runtime::{ RunSocket, RunStream };

use std::convert::TryInto;

/// listen(quic_socket)
#[rustler::nif]
fn listen(quic_socket: ElixirInterface) -> Result<Socket, Error> {
    Socket::new(quic_socket, ConnType::Server, None)
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(quic_socket: ElixirInterface, address: SocketAddr, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = Socket::new(quic_socket, ConnType::Client, None)?;
    (**socket).lock().unwrap().connect(*address, timeout)?;
    Ok(socket)
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Socket, timeout: Option<u64>) -> Result<Socket, Error> {
    let socket = (**quic_socket).lock().unwrap().accept(timeout)?
        .into();
    Ok(socket)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Socket, stream_type: StreamType) -> Result<Stream, Error> {
    let stream : Stream = match stream_type {
        StreamType::Uni => (**quic_socket).lock().unwrap().new_uni_stream()?.into(),
        StreamType::Bi => (**quic_socket).lock().unwrap().new_bi_stream()?.into(),
    };
    Ok(stream)
}

/// read(quic_stream, amount)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>, Error> {
    if let Ok(amount) = amount.try_into() {
        let mut buffer = Vec::with_capacity(amount);
        match (**quic_stream).lock().unwrap().read(&mut buffer, timeout) {
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
    (**quic_stream).lock().unwrap().write(&buffer)
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Socket, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
    (**quic_socket).lock().unwrap().close(error_code, reason)
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
    (**quic_stream).lock().unwrap().close_stream(error_code, reason)
}

