mod error;
mod options;
mod net;
use error::{ ApplicationError };
use options::{ QuicOptions };
use net::Net;

#[macro_use]
extern crate rustler;
use rustler::{ Decoder, LocalPid, Term };
use rustler_codegen::{ NifStruct, NifTuple, NifUnitEnum };

use quinn::{ EndpointBuilder, NewConnection };

use std::{
    task::{ Context },
};

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

type Port = u16;

// Override the standard net::IpAddr so that we can easily translate between Langs
#[derive(NifTuple)]
#[rustler(encode, decode)]
struct Ipv4Addr(u8, u8, u8, u8);

struct SocketAddr(std::net::SocketAddr);

#[derive(NifStruct)]
#[module="QuicSocket"]
#[rustler(encode, decode)]
pub struct ElixirInterface {
    socket: Option<QuicSocket>,
    socket_addr: Option<SocketAddr>,
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


impl<'a> Decoder<'a> for SocketAddr {
    fn decode(term : Term<'a>) -> Result<SocketAddr, rustler::Error> {
        let terms = tuple::get_tuple(term)?;
        let ip = tuple::get_tuple(terms.0)?;
        let port = terms.1;
        
        
    }
}


impl Net for ElixirInterface {
    fn address(&self) -> &std::net::SocketAddr {
        &self.socket_addr
            .unwrap()
            .0
    }

    fn configure_client(&self) -> EndpointBuilder {

    }

    fn configure_server(&self) -> EndpointBuilder {

    }

    fn notify(&self, connection : NewConnection, ctx : &mut Context) {
        
    }

    fn server_name(&self) -> &str {

    }
}

