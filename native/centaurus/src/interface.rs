use crate::api;

#[macro_use]
extern crate rustler;
use rustler::{ Encoder, Env, Error, Term, Pid };
use serde::{ Serialize, Deserialize };
use serde_rustler::{ to_term, from_term };
use std::{
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename="Elixir.Centaurus.Types.QuicSocket")]
pub struct ElixirInterface <'a> {
    socket: Option<QuicSocket>,
    ip_addr: Option<IPAddr>,
    port: Option<Port>,
    server_name: &'a str,
    options: Vec<QuicOptions>,
    certificates: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename="Elixir.Centaurus.Types.QuicSocket.QuicStream")]
pub struct ElixirStream <'a> {
    stream_id: QuicStream,
    socket_id: QuicSocket,
    direction: Direction,
    data: &'a str,
    options: Vec<QuicOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename="bi")]
    Bi,
    #[serde(rename="uni")]
    Uni,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuicStream(Pid);

#[derive(Debug, Serialize, Deserialize)]
struct QuicSocket(Pid);

/// accept(quic_socket, timeout)
fn accept<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// connect(quic_socket, timeout)
fn connect<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// close(quic_socket, error_code)
fn close<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// close_stream(quic_stream, error_code)
fn close_stream<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// listen(quic_socket)
fn listen<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// open_stream(quic_socket, direction)
fn open_stream<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// read(quic_stream, timeout)
fn read<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

/// write(quic_stream, data)
fn write<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result {

}

