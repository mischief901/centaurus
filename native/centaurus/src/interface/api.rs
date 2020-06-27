//! The Elixir entrypoint.
use super::types::{
    BeamSocket,
    BeamStream,
    Error,
    NewSocket,
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

/// start()
/// Must be called before running any functions.
#[rustler::nif]
fn start() {
    conn::start_runtime()
}

/// listen(socket_config, stream_config)
#[rustler::nif]
fn listen(socket_config: BeamSocket, stream_config: BeamStream) -> Result<NewSocket> {
    let socket = conn::NewSocket::new(SocketType::Server, socket_config.into(), stream_config.into())?;
    Ok(socket.into())
}

/// connect(socket_config, stream_config, address, timeout)
#[rustler::nif]
fn connect(socket_config: BeamSocket, stream_config: BeamStream, address: SocketAddr, timeout: Option<u64>) -> Result<Socket> {
    let socket = conn::NewSocket::new(SocketType::Client, socket_config.into(), stream_config.into())?
        .connect(*address, timeout)?;
//        .context("Connect Failure.")?;
    Ok(socket.into())
}

/// accept(socket, timeout)
#[rustler::nif]
fn accept(quic_socket: NewSocket, timeout: Option<u64>) -> Result<Socket> {
    let socket = quic_socket.accept(timeout)?
//        .context("Accept Failure.")?
        .into();
    Ok(socket)
}

/// open_stream(socket, stream_type)
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

/// read(stream, amount, timeout)
#[rustler::nif]
fn read(quic_stream: Stream, amount: u64, timeout: Option<u64>) -> Result<(usize, Vec<u8>)> {
    let amount = amount.try_into().context("Invalid Read amount.")?;
    let buffer = Vec::with_capacity(amount);
    match quic_stream.read(buffer.clone(), timeout) {
        Ok(length) => Ok((length, buffer)),
        Err(error) => Err(error).context("Read Error.").map_err(|err| err.into()),
    }
}

/// write(stream, data)
#[rustler::nif]
fn write<'a>(quic_stream: Stream, data: &'a str) -> Result<()> {
    let buffer = Vec::from(data);
    quic_stream.write(buffer).context("Write Error.")?;
    Ok(())
}

/// close(socket, error_code, reason)
#[rustler::nif]
fn close<'a>(quic_socket: Socket, error_code: ApplicationError, reason: Option<&'a str>) -> Result<()> {
    quic_socket.close(error_code, reason.map(|s| s.to_owned()))
        .context("Could not close socket.")?;
    Ok(())
}

/// close_stream(stream, error_code)
#[rustler::nif]
fn close_stream(quic_stream: Stream, error_code: ApplicationError) -> Result<()> {
    quic_stream.close_stream(error_code)
        .context("Could not close stream.")?;
    Ok(())
}

