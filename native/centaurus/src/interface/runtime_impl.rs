//! Contains the RunSocket and RunStream impls. See centaurus::runtime for details.

use super::types::{
    ElixirInterface,
    SocketHandler,
    SocketRef,
    Stream,
    StreamHandler,
};

use crate::error::{ ApplicationError, Error };
use crate::runtime::{ Handle, RunSocket, RunStream, Unlockable };

use quinn::{ Endpoint, EndpointDriver, Incoming };

use rustler::{ ResourceArc };

use std::sync::Mutex;

impl Unlockable for SocketHandler {
    type Unlocked = Handle;
    type LockError = Error;

    fn unlock(&self) -> Result<&mut Self::Unlocked, Self::LockError> {
        unimplemented!();
    }
}

impl RunSocket for SocketHandler {
    type Socket = ElixirInterface;
    type Stream = Stream;
    type Error = Error;

    fn accept(&self, timeout: Option<u64>) -> Result<Self::Socket, Self::Error> {
        unimplemented!();
    }

    fn connect(&self, timeout: Option<u64>) -> Result<&Self, Self::Error> {
        unimplemented!();
    }
    
    fn new_uni_stream(&self) -> Result<Self::Stream, Self::Error> {
        unimplemented!();
    }

    fn new_bi_stream(&self) -> Result<Self::Stream, Self::Error> {
        unimplemented!();
    }

    fn new_peer_uni_stream(&self) -> Result<Self::Stream, Self::Error> {
        unimplemented!();
    }

    fn new_peer_bi_stream(&self) -> Result<Self::Stream, Self::Error> {
        unimplemented!();
    }

    fn close(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

impl Unlockable for StreamHandler {
    type Unlocked = Handle;
    type LockError = Error;

    fn unlock(&self) -> Result<&mut Self::Unlocked, Self::LockError> {
        unimplemented!();
    }
}

impl RunStream for StreamHandler {
    type Error = Error;

    fn new() -> Self {
        unimplemented!();
    }
    
    fn read(&self, buffer: &[u8], timeout: Option<u64>) -> Result<u64, Self::Error> {
        unimplemented!();
    }

    fn write(&self, buffer: &[u8]) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
        unimplemented!();
    }   
}

impl From<(EndpointDriver, Endpoint, Incoming)> for SocketHandler {
    fn from(new_conn: (EndpointDriver, Endpoint, Incoming)) -> Self {
        let handle : Handle = new_conn.into();
        Self(Mutex::new(handle))
    }
}

