//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs, SocketType, SocketConfig, StreamConfig, StreamType };
use crate::conn::{ Socket, Stream };
use crate::error::{ ApplicationError, Error };
use crate::state::{ SocketState, StreamState };

use quinn::{ OpenBi, OpenUni };

use tokio::runtime;
use tokio::{
    runtime::{ Builder },
    sync::{
        mpsc::{
            unbounded_channel,
            UnboundedReceiver as AsyncReceiver,
            UnboundedSender as AsyncSender,
        },
        Mutex,
    },
    task::{ JoinHandle },
};

use std::{
    future::{ Future },
    marker::{ PhantomData },
    net::{ SocketAddr },
    sync::{ Arc },
    sync::mpsc::{ Sender },
    time::{ Duration },
};

lazy_static! {
    static ref POOL_HANDLE : Option<JoinHandle<()>> = None;
}

/// Starts a new tokio runtime.
/// The runtime is a threaded pool named "centaurus-pool".
fn new_pool() -> runtime::Runtime {
    Builder::new()
        .thread_name("centaurus-pool")
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
}


// The main tokio runtime.
struct RuntimeInternal(runtime::Runtime);

// State of the socket on the runtime.
struct SocketRuntime<S : SocketConfig, T : StreamConfig> {
    receiver: AsyncReceiver<SocketEvent<S, T>>,
    configs: Configs<S, T>,
    socket_state: SocketState,
}

// State of the stream on the runtime.
struct StreamRuntime<S : SocketConfig, T : StreamConfig> {
    receiver: AsyncReceiver<StreamEvent<T>>,
    configs: Configs<S, T>,
    stream_state: StreamState,
}

    
pub enum Event<S : SocketConfig, T : StreamConfig> {
    OpenSocket(Sender<Result<Socket<S, T>, Error>>, SocketType, Configs<S, T>, SocketState),
}

// Events the socket knows how to handle.
pub enum SocketEvent<S : SocketConfig, T : StreamConfig> {
    Accept(Sender<Result<Socket<S, T>, Error>>, Option<Duration>),
    Close(ApplicationError, Option<Vec<u8>>),
    Connect(Sender<Result<Socket<S, T>, Error>>, SocketAddr, Option<Duration>),
    Listen(Sender<Result<(), Error>>, SocketAddr),
    OpenBiStream(Sender<Result<Stream<T>, Error>>),
    OpenUniStream(Sender<Result<Stream<T>, Error>>),
}

// Events the stream knows how to handle.
pub enum StreamEvent<T> {
    CloseStream(ApplicationError, Option<Vec<u8>>),
    Read(Sender<Result<u64, Error>>, Arc<Mutex<Vec<u8>>>, Option<Duration>),
    Write(Sender<Result<(), Error>>, Vec<u8>),
    Blank(PhantomData<T>),
}

pub fn run<S : SocketConfig + 'static, T : StreamConfig + 'static>(mut pool : AsyncReceiver<Event<S, T>>) {
    let rt = new_pool();
    rt.block_on(async {
        loop {
            tokio::join!{
                tokio::spawn(async move {
                    match pool.recv().await.unwrap() {
                        Event::OpenSocket(responder, conn_type, configs, socket_state) => {
                            let (sender, receiver) = unbounded_channel();
                            let socket = SocketRuntime {
                                receiver,
                                configs,
                                socket_state,
                            };
                            // Send the socket's sender to the synchronous side.
                            responder.send(Ok(sender.into())).unwrap();
                            // Spawn a new task to handle the socket.
                            tokio::spawn(async move {
                                run_socket(socket);
                            });
                        }
                    }
                })
            };
        }
    });
}


async fn run_socket<S, T>(mut socket: SocketRuntime<S, T>)
where S : SocketConfig, T : StreamConfig {
    loop {
        match socket.receiver.recv().await.unwrap() {
            SocketEvent::Accept(response, timeout) => {},
            SocketEvent::Close(application_error, reason) => {},
            SocketEvent::Connect(response, sock_addr, timeout) => {},
            SocketEvent::Listen(response, sock_addr) => {},
            SocketEvent::OpenBiStream(response) => {},
            SocketEvent::OpenUniStream(response) => {},
        }
    }
}

async fn run_stream<S, T>(mut stream: StreamRuntime<S, T>)
where S : SocketConfig, T : StreamConfig {
    loop {
        match stream.receiver.recv().await.unwrap() {
            StreamEvent::CloseStream(application_error, reason) => {},
            StreamEvent::Read(response, buffer, timeout) => {},
            StreamEvent::Write(response, buffer) => {},
        }
    }
}

