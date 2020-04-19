//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs };
use crate::conn::{ Socket, Stream };
use crate::error::{ ApplicationError };
use crate::state::{ SocketState, StreamState };
use crate::interface::types::{ SocketType };

use anyhow::{ Context, Result };

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
};

use std::{
    net::{ SocketAddr },
    sync::{ Arc, Once },
    sync::mpsc::{ Sender },
    time::{ Duration },
};

type Responder<T> = Mutex<Sender<T>>;

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

#[derive(Debug)]
// State of the socket on the runtime.
struct SocketRuntime {
    receiver: AsyncReceiver<SocketEvent>,
    configs: Configs,
    socket_state: SocketState,
}

#[derive(Debug)]
// State of the stream on the runtime.
struct StreamRuntime {
    receiver: AsyncReceiver<StreamEvent>,
    configs: Configs,
    stream_state: StreamState,
}

pub fn handle() -> Result<AsyncSender<Event>> {
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

#[derive(Debug)]
pub enum Event {
    OpenSocket(Responder<Result<Socket>>, SocketType, Configs, SocketState),
}

#[derive(Debug)]
// Events the socket knows how to handle.
pub enum SocketEvent {
    Accept(Responder<Result<Socket>>, Option<Duration>),
    Close(ApplicationError, Option<Vec<u8>>),
    Connect(Responder<Result<Socket>>, SocketAddr, Option<Duration>),
    Listen(Responder<Result<()>>),
    OpenBiStream(Responder<Result<Stream>>),
    OpenUniStream(Responder<Result<Stream>>),
}

#[derive(Debug)]
// Events the stream knows how to handle.
pub enum StreamEvent {
    CloseStream(ApplicationError, Option<Vec<u8>>),
    Read(Responder<Result<u64>>, Arc<Mutex<Vec<u8>>>, Option<Duration>),
    Write(Responder<Result<()>>, Vec<u8>),
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
                            responder.into_inner().send(Ok(sender.into())).unwrap();
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
            SocketEvent::Listen(response) => {},
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

