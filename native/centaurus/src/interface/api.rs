//! The Elixir entrypoint.
use super::types;
use super::types::{
    BeamSocket,
    BeamStream,
    Error,
    Socket,
    SocketType,
    SocketAddr,
    Stream,
    StreamType,
};

use crate::conn;
use crate::error::{ ApplicationError };

use anyhow::{ Context };

use std::convert::TryInto;

type Result<T> = std::result::Result<T, Error>;

/// listen(quic_socket)
#[rustler::nif]
fn listen(socket_config: BeamSocket, stream_config: BeamStream) -> Result<()> {
    let socket = conn::Socket::new(SocketType::Server, socket_config.into(), stream_config.into())?;
    socket.listen()
        .context("Listen Failure.")?;
    Ok(())
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect(socket_config: BeamSocket, stream_config: BeamStream, address: SocketAddr, timeout: Option<u64>) -> Result<Socket> {
    let socket = conn::Socket::new(SocketType::Client, socket_config.into(), stream_config.into())?;
    let socket = socket.connect(*address, timeout)
        .context("Connect Failure.")?;
    Ok(socket.into())
}

/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept(quic_socket: Socket, timeout: Option<u64>) -> Result<Socket> {
    let socket = quic_socket.accept(timeout)
        .context("Accept Failure.")?
        .into();
    Ok(socket)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream(quic_socket: Socket, stream_type: StreamType) -> Result<Stream> {
    let stream : Stream = match stream_type {
        StreamType::Uni => {
            quic_socket.new_uni_stream()
                .context("Error opening new unidirectional stream.")?
                .into()
        },
        StreamType::Bi => {
            quic_socket.new_bi_stream()
                .context("Error opening new bidirectional stream.")?
                .into()
        },
    };
    Ok(stream)
}

/// read(quic_stream, amount, timeout)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<Vec<u8>> {
    let amount = amount.try_into().context("Invalid Read amount.")?;
    let buffer = Vec::with_capacity(amount);
    match quic_stream.read(buffer.clone(), timeout) {
        Ok(_length) => Ok(buffer),
        Err(error) => Err(error).context("Read Error.").map_err(|err| err.into()),
    }
}

/// write(quic_stream, data)
#[rustler::nif]
fn write<'a>(quic_stream: Stream, data: &'a str) -> Result<()> {
    let buffer = Vec::from(data);
    quic_stream.write(buffer).context("Write Error.")?;
    Ok(())
}

/// close(quic_socket, error_code, reason)
#[rustler::nif]
fn close(quic_socket: Socket, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<()> {
    quic_socket.close(error_code, reason)
        .context("Could not close socket.")?;
    Ok(())
}

/// close_stream(quic_stream, error_code, reason)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<()> {
    quic_stream.close_stream(error_code, reason)
        .context("Could not close stream.")?;
    Ok(())
}

