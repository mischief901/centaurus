//! Contains the Encode and Decode functions.

use super::types::{
    Certificates,
    Connection,
    ElixirInterface,
    ElixirStream,
    PrivateKey,
    SocketAddr,
    SocketHandler,
    Stream,
    StreamHandler,
};

use rustler::{ Decoder, Encoder, Env, ResourceArc, Term };
use rustler::types::tuple;

use std::path::PathBuf;

impl<'a> Decoder<'a> for SocketAddr {
    fn decode(term : Term<'a>) -> Result<SocketAddr, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let sock_addr : std::net::SocketAddr = raw
            .parse()
            .or(Err(rustler::Error::BadArg))?;
        Ok(SocketAddr(sock_addr))
    }
}

impl<'a> Encoder for SocketAddr {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let SocketAddr(sock_addr) = self;
        sock_addr.to_string().encode(env)
    }
}

impl<'a> Decoder<'a> for Certificates {
    fn decode(term : Term<'a>) -> Result<Self, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let mut path = PathBuf::new();
        path.push(raw);
        Ok(Certificates(path))
    }
}

impl<'a> Encoder for Certificates {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let Certificates(path) = self;
        path.to_str().encode(env)
    }
}

impl<'a> Decoder<'a> for PrivateKey {
    fn decode(term : Term<'a>) -> Result<Self, rustler::Error> {
        let raw : &str = Decoder::decode(term)?;
        let mut path = PathBuf::new();
        path.push(raw);
        Ok(PrivateKey(path))
    }
}

impl<'a> Encoder for PrivateKey {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let PrivateKey(path) = self;
        path.to_str().encode(env)
    }
}
/*
impl<'a> Decoder<'a> for &'a Connection {
    fn decode(term: Term<'a>) -> Result<Self, rustler::Error> {
        match tuple::get_tuple(term) {
            Err(error) => Err(error),
            Ok(tuple) => {
                let meta : ElixirInterface = Decoder::decode(tuple[0])?;
                let conn : SocketHandler = Decoder::decode(tuple[1])?;
                Ok(&Connection{
                    meta,
                    conn
                })
            }
        }
    }
}
/*
impl<'a> Encoder for &'a Connection {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        (*self).encode(env)
    }
}
*/
impl<'a> Decoder<'a> for &'a Stream {
    fn decode(term: Term<'a>) -> Result<Self, rustler::Error> {
        match tuple::get_tuple(term) {
            Err(error) => Err(error),
            Ok(tuple) => {
                let meta : ElixirStream = Decoder::decode(tuple[0])?;
                let conn : StreamHandler = Decoder::decode(tuple[1])?;
                Ok(&Stream{
                    meta,
                    conn
                })
            }
        }
    }
}

impl<'a> Encoder for &'a Stream {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        (*self).encode(env)
    }
}
*/
