mod api;

#[macro_use]
extern crate rustler;
use rustler::{ Atom, Encoder, Env, LocalPid, Term };
use serde::{ Serialize, Deserialize };
use serde_rustler::{ to_term, from_term };
use std::{
    path::PathBuf,
};

mod error;
mod options;
mod net;
use error::{
    Error,
    ApplicationError,
};
use options::{ QuicOptions, };

type Port = u32;

type IPAddr = (u8, u8, u8, u8);

type Result<'a> = std::result::Result<Term<'a>, serde_rustler::Error>;

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
struct QuicStream(LocalPid);

#[derive(Debug, Serialize, Deserialize)]
struct QuicSocket(LocalPid);


/// accept(quic_socket, timeout)
#[rustler::nif]
fn accept<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_socket : ElixirInterface = from_term(args[0])?;
    let timeout : u64 = from_term(args[1])?;
    
    to_term(env, quic_socket)
}

/// connect(quic_socket, timeout)
#[rustler::nif]
fn connect<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_socket : ElixirInterface = from_term(args[0])?;
    let timeout : u64 = from_term(args[1])?;

    to_term(env, quic_socket)
}

/// close(quic_socket, error_code)
#[rustler::nif]
fn close<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_socket : ElixirInterface = from_term(args[0])?;
    let error_code : ApplicationError = from_term(args[1])?;

    to_term(env, ())
}

/// close_stream(quic_stream, error_code)
#[rustler::nif]
fn close_stream<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_stream : ElixirStream = from_term(args[0])?;
    let error_code : ApplicationError = from_term(args[1])?;

    to_term(env, ())
}

/// listen(quic_socket)
#[rustler::nif]
fn listen<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_stream : ElixirInterface = from_term(args[0])?;

    to_term(env, quic_stream)
}

/// open_stream(quic_socket, direction)
#[rustler::nif]
fn open_stream<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_socket : ElixirInterface = from_term(args[0])?;
    let direction : ApplicationError = from_term(args[1])?;

    to_term(env, quic_socket)
}

/// read(quic_stream, timeout)
#[rustler::nif]
fn read<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_stream : ElixirStream = from_term(args[0])?;
    let timeout : u64 = from_term(args[1])?;

    to_term(env, quic_stream)
}

/// write(quic_stream, data)
#[rustler::nif]
fn write<'a>(env: Env<'a>, args: &Vec<Term<'a>>) -> Result<'a> {
    let mut quic_stream : ElixirStream = from_term(args[0])?;
    let data : &str = from_term(args[1])?;

    to_term(env, quic_stream)
}


