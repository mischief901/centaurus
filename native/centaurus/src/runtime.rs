/// Provides traits and types for working with the Tokio runtime.
use crate::error::{ ApplicationError, Error };

use tokio::runtime::{ Builder };

use quinn::{ NewConnection };

use std::sync::Mutex;

pub struct Runtime(tokio::runtime::Runtime);
pub struct Handle(Mutex<tokio::runtime::Handle>);

pub trait RunSocket {
    // The Socket type to return on successfully opening a connection
    type Socket;
    // The Stream type to return on successfully opening a new stream
    type Stream;
    
    // Accept a new connection
    fn accept(&self, timeout: Option<u64>) -> Self::Socket;

    // Open a new unidirectional stream
    fn new_uni_stream(&self) -> Self::Stream;

    // Open a new bidirectional stream
    fn new_bi_stream(&self) -> Self::Stream;

    // What to do when a peer opens a new unidirectional stream
    fn new_peer_uni_stream(&self) -> Self::Stream;

    // What to do when a peer opens a new bidirectional stream
    fn new_peer_bi_stream(&self) -> Self::Stream;
    
    // Close the connection
    fn close(&self, error_code: ApplicationError, reason: Vec<u8>);
}


pub trait RunStream {
    type Stream;
    
    fn read(&self, buffer: &[u8], timeout: Option<u64>) -> Result<Vec<u8>, Error>;

    fn write(&self, buffer: &[u8]) -> Result<(), Error>;
    
    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>);
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


