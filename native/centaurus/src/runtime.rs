/// Provides traits and types for working with the Tokio runtime.
use crate::error::{ ApplicationError, Error };

use tokio::runtime::{ Builder };

use quinn::{ Endpoint, EndpointDriver, Incoming };

pub struct Runtime(tokio::runtime::Runtime);
pub struct Handle(tokio::runtime::Handle);


impl From<(EndpointDriver, Endpoint, Incoming)> for Handle {
    fn from((driver, endpoint, incoming) : (EndpointDriver, Endpoint, Incoming)) -> Self {
        unimplemented!();
    }
}

pub trait Unlockable {
    type Unlocked;
    type LockError;
    
    fn unlock(&self) -> Result<&mut Self::Unlocked, Self::LockError>;
}

pub trait RunSocket : Unlockable {
    // The Socket type to return on successfully opening a connection
    type Socket;
    // The Stream type to return on successfully opening a new stream
    type Stream;
    // The error type to return on error.
    type Error;
    
    // Accept a new connection
    fn accept(&self, timeout: Option<u64>) -> Result<Self::Socket, Self::Error>;

    // Connect to a server
    fn connect(&self, timeout: Option<u64>) -> Result<&Self, Self::Error>;
    
    // Open a new unidirectional stream
    fn new_uni_stream(&self) -> Result<Self::Stream, Self::Error>;

    // Open a new bidirectional stream
    fn new_bi_stream(&self) -> Result<Self::Stream, Self::Error>;

    // What to do when a peer opens a new unidirectional stream
    fn new_peer_uni_stream(&self) -> Result<Self::Stream, Self::Error>;

    // What to do when a peer opens a new bidirectional stream
    fn new_peer_bi_stream(&self) -> Result<Self::Stream, Self::Error>;
    
    // Close the connection
    fn close(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Self::Error>;
}


pub trait RunStream : Unlockable {
    // The error type to return on error.
    type Error;

    fn new() -> Self;
    
    fn read(&self, buffer: &[u8], timeout: Option<u64>) -> Result<u64, Self::Error>;

    fn write(&self, buffer: &[u8]) -> Result<(), Self::Error>;
    
    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>)-> Result<(), Self::Error>;
}

impl Runtime {
    pub fn new() -> Self {
        let rt = Builder::new()
            .thread_name("centaurus-pool")
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        Runtime(rt)
    }
}


