//! Provides traits and types for working with the Tokio runtime.
use crate::error::{ ApplicationError, Error };

use quinn::{ OpenBi, OpenUni };

use tokio::runtime;
use tokio::runtime::{ Builder, Handle };
use tokio::task::{ JoinHandle };

use std::{
    future::{ Future },
    net::{ SocketAddr },
    ops::{ Deref, DerefMut },
    sync::{ Arc, Mutex },
};

/// A newtype for the tokio Handle to the Runtime.
#[derive(Clone)]
pub struct Runtime(Arc<Mutex<runtime::Runtime>>);

/// The RunSocket trait dictates how to handle a QUIC socket in the runtime.
/// 
pub trait RunSocket<S : Send + Sync> : Send + Sync + Sized {
    /// The Stream type to return on successfully opening a new stream
    type Stream: Send + Sync;

    /// Loops over the incoming uni and bi streams for the socket.
    /// This should know how to communicate that a new stream is opened to the socket owner.
    fn run_socket(&self) -> Result<(), Error>;
    
    /// Accept a new connection.
    /// Takes in an optional timeout parameter (u64).
    /// Returns Ok(Socket) when a new connection has been established.
    /// Returns Err(Error) on timeout or internal error.
    fn accept(&self, timeout: Option<u64>) -> Result<Self, Error>;

    /// Connect to a server
    /// Takes in an optional timeout parameter (u64).
    /// Returns Ok(self) when the connection has been established.
    /// Returns Err(Error) on timeout or internal error.
    fn connect(&self, address: SocketAddr, timeout: Option<u64>) -> Result<(), Error>;
    
    /// Open a new unidirectional stream
    /// Returns Ok(Stream) when a new stream is successfully opened.
    /// Returns Err(Error) when there is an error opening the stream.
    fn new_uni_stream(&self) -> Result<Self::Stream, Error>;

    /// Open a new bidirectional stream
    /// Returns Ok(Stream) when a new stream is successfully opened.
    /// Returns Err(Error) when there is an error opening the stream.
    fn new_bi_stream(&self) -> Result<Self::Stream, Error>;
    
    /// Close the QUIC connection and all associated streams.
    /// Takes in the ApplicationError and Reason to send to the peer.
    /// An error code of 0 indicates no error.
    /// Returns Ok(()) on success.
    /// Return Err(Error) on internal error.
    fn close(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error>;
}

/// The RunStream trait dictates how to handle a QUIC stream in the runtime.
///
pub trait RunStream<S, T> : Sync + Send {
    
    /// Loop over the recv stream and the close stream future.
    fn run_stream(self) -> Result<(), Error>;
    
    /// Creates a new stream of the appropriate type.
    /// These streams are owned by the peer and cannot be closed by anyone else.
    fn new_uni_stream(stream_config: Option<T>, stream_future: OpenUni) -> Self;
    fn new_bi_stream(stream_config: Option<T>, stream_future: OpenBi) -> Self;
    
    /// Read from the stream.
    /// Takes in a mutable buffer into which to write data and an optional timeout (u64).
    /// Returns Ok(u64) for success. The u64 is the number of bytes read.
    /// Returns Err(Error) on timeout, stream type mismatch, or internal error.
    fn read(&self, buffer: &mut [u8], timeout: Option<u64>) -> Result<u64, Error>;

    /// Write to the stream.
    /// Takes in a buffer from which to read data.
    /// Returns Ok(()) on success.
    /// Returns Err(Error) on stream type mismatch or internal error.
    fn write(&self, buffer: &[u8]) -> Result<(), Error>;

    /// Closes the stream.
    /// Takes in an ApplicationError code and a reason to send to the peer.
    /// Returns Ok(()) on success.
    /// Returns Err(Error) on stream type mismatch or internal error.
    /// An ApplicationError code of 0 indicates no error to the peer.
    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>)-> Result<(), Error>;
}

impl Runtime {
    /// Starts a new tokio runtime.
    /// The runtime is a threaded pool named "centaurus-pool".
    pub fn new() -> Self {
        let rt = Builder::new()
            .thread_name("centaurus")
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        Runtime(Arc::new(Mutex::new(rt)))
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where F : Future + Send + 'static,
          F::Output : Send + 'static {
        (*self).lock().unwrap().spawn(future)
    }
    
    pub fn enter<F, R>(&self, function: F) -> R
    where F : FnOnce() -> R {
        (*self).lock().unwrap().enter(function)
    }

    pub fn handle(&self) -> Handle {
        (*self).lock().unwrap().handle().clone()
    }

    pub fn block_on<F : Future>(&self, future: F) -> F::Output {
        (*self).lock().unwrap().block_on(future)
    }
}

impl Deref for Runtime {
    type Target = Mutex<runtime::Runtime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

