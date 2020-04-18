//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs, SocketConfig, StreamConfig };
use crate::conn::{ Socket, Stream };
use crate::error::{ ApplicationError, Error };
use crate::state::{ SocketState, StreamState };

use crate::interface::types::{ SocketType, StreamType };

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
    sync::{ Arc, Once },
    sync::mpsc::{ Sender },
    time::{ Duration },
};

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
struct SocketRuntime {
    receiver: AsyncReceiver<SocketEvent>,
    configs: Configs,
    socket_state: SocketState,
}

// State of the stream on the runtime.
struct StreamRuntime {
    receiver: AsyncReceiver<StreamEvent>,
    configs: Configs,
    stream_state: StreamState,
}

pub fn handle() -> Result<AsyncSender<Event>, Error> {
    static mut SENDER : Option<AsyncSender<Event>> = None;
    static INIT : Once = Once::new();
    let sender = unsafe {
        INIT.call_once(|| {
            let (sender, receiver) : (AsyncSender<Event>,
                                      AsyncReceiver<Event>) = unbounded_channel();
            SENDER = Some(sender);
            std::thread::spawn(move || {
                run(receiver);
            });
        });
        SENDER.as_ref().unwrap().clone()
    };
    Ok(sender)
}
    
pub enum Event {
    OpenSocket(Sender<Result<Socket, Error>>, SocketType, Configs, SocketState),
}

// Events the socket knows how to handle.
pub enum SocketEvent {
    Accept(Sender<Result<Socket, Error>>, Option<Duration>),
    Close(ApplicationError, Option<Vec<u8>>),
    Connect(Sender<Result<Socket, Error>>, SocketAddr, Option<Duration>),
    Listen(Sender<Result<(), Error>>, SocketAddr),
    OpenBiStream(Sender<Result<Stream, Error>>),
    OpenUniStream(Sender<Result<Stream, Error>>),
}

// Events the stream knows how to handle.
pub enum StreamEvent {
    CloseStream(ApplicationError, Option<Vec<u8>>),
    Read(Sender<Result<u64, Error>>, Arc<Mutex<Vec<u8>>>, Option<Duration>),
    Write(Sender<Result<(), Error>>, Vec<u8>),
}

pub fn run(mut pool : AsyncReceiver<Event>) {
    let mut rt = new_pool();
    rt.block_on(async {
        let result = tokio::join!{
            tokio::spawn(async move {
                loop {
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
                                run_socket(socket).await;
                            });
                        }
                    }
                }
            })
        };
        result.0.unwrap();
    });
}

async fn run_socket(mut socket: SocketRuntime) {
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

async fn run_stream(mut stream: StreamRuntime) {
    loop {
        match stream.receiver.recv().await.unwrap() {
            StreamEvent::CloseStream(application_error, reason) => {},
            StreamEvent::Read(response, buffer, timeout) => {},
            StreamEvent::Write(response, buffer) => {},
        }
    }
}

