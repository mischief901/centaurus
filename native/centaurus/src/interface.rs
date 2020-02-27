#[macro_use]
extern crate rustler;
use rustler::{ LocalPid };
use rustler_codegen::{ NifStruct, NifTuple, NifUnitEnum };

mod error;
mod options;
mod net;
use error::{
    ApplicationError,
};
use options::{ QuicOptions, };

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

type Port = u32;

#[derive(NifTuple)]
#[rustler(encode, decode)]
struct IPAddr(u8, u8, u8, u8);

#[derive(NifStruct)]
#[module="QuicSocket"]
#[rustler(encode, decode)]
pub struct ElixirInterface {
    socket: Option<QuicSocket>,
    ip_addr: Option<IPAddr>,
    port: Option<Port>,
    server_name: String,
    options: Vec<QuicOptions>,
    certificates: Option<String>,
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


