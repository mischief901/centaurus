//! Contains the Encode and Decode functions.

use super::types::{
    Certificates,
    PrivateKey,
    SocketAddr,
};

use rustler::{ Decoder, Encoder, Env, Term };

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

