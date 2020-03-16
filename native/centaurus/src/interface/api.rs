//! The Elixir entrypoint.

use super::types::{
    Connection,
    ElixirInterface,
    ElixirStream,
    Stream,
    StreamType,
};

use crate::error::{ ApplicationError, Error };
use crate::runtime::{ RunSocket, RunStream };

use std::convert::TryInto;

/// listen(quic_socket)
#[rustler::nif]
fn listen(quic_socket: ElixirInterface) -> Result<Connection, Error> {
    Connection::new_server(quic_socket)
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(quic_socket: ElixirInterface, timeout: Option<u64>) -> Result<Connection, Error> {
    let conn = Connection::new_client(quic_socket)?;
    conn.inner()
        .handler()
        .connect(timeout)?;
    Ok(conn)
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Connection, timeout: Option<u64>) -> Result<Connection, Error> {
    quic_socket.inner()
        .handler()
        .accept(timeout)?;
    Ok(quic_socket)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Connection, stream_type: StreamType) -> Result<Stream, Error> {
    match stream_type {
        StreamType::Uni => quic_socket.inner().handler().new_uni_stream(),
        StreamType::Bi => quic_socket.inner().handler().new_bi_stream(),
    }
}

/// read(quic_stream, amount)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>, Error> {
    if let Ok(amount) = amount.try_into() {
        let mut buffer = Vec::with_capacity(amount);
        match quic_stream.inner().handler().read(&buffer, timeout) {
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
    let mut buffer = Vec::from(data);
    quic_stream.inner().handler().write(&buffer)
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Connection, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
    quic_socket.inner().handler().close(error_code, reason)
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
    quic_stream.inner().handler().close_stream(error_code, reason)
}

