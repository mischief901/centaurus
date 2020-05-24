//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs };
use crate::conn::{ Socket, Stream };
use crate::error::{ ApplicationError };
use crate::state::{ SocketState, StreamState };
use crate::interface::types::{ SocketType };

use anyhow::{ Context, Result };

use quinn::{ IncomingBiStreams, IncomingUniStreams, OpenBi, OpenUni };

use tokio::runtime;
use tokio::{
    runtime::{ Builder },
    stream::{ StreamExt },
    sync::{
        mpsc::{
            unbounded_channel,
            UnboundedReceiver as AsyncReceiver,
            UnboundedSender as AsyncSender,
        },
        Mutex,
    },
    time,
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

#[derive(Clone, Debug)]
// State of the socket on the runtime.
struct SocketRuntime {
    configs: Configs,
    state: SocketState,
}

#[derive(Debug)]
struct SocketRuntimeLocal {
    receiver: AsyncReceiver<SocketEvent>,
    runtime: SocketRuntime,
}

impl SocketRuntimeLocal {
    fn new(runtime: SocketRuntime) -> (AsyncSender<SocketEvent>, Self) {
        let (sender, receiver) = unbounded_channel();
        let new_socket = Self {
            receiver,
            runtime,
        };
        (sender, new_socket)
    }
}

#[derive(Clone, Debug)]
// State of the stream on the runtime.
struct StreamRuntime {
    configs: Configs,
    state: StreamState,
}

#[derive(Debug)]
struct StreamRuntimeLocal {
    receiver: AsyncReceiver<StreamEvent>,
    runtime: StreamRuntime,
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
    Connect(Responder<Result<()>>, SocketAddr, Option<Duration>),
    Listen(Responder<Result<Socket>>, SocketAddr),
    OpenBiStream(Responder<Result<Stream>>),
    OpenUniStream(Responder<Result<Stream>>),
}

#[derive(Debug)]
// Events the stream knows how to handle.
pub enum StreamEvent {
    CloseStream(ApplicationError),
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
                        Event::OpenSocket(responder, conn_type, configs, state) => {
                            let runtime = SocketRuntime {
                                configs,
                                state,
                            };
                            let (sender, socket) = SocketRuntimeLocal::new(runtime);
                            // Send the socket's sender to the synchronous side.
                            responder.into_inner()
                                .send(Ok(sender.into()))
                                .unwrap();
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

async fn run_socket(socket: SocketRuntimeLocal) {
    let peer = peer_socket_event(socket.runtime.clone());
    let local = local_socket_event(socket);
    tokio::pin!(local, peer);
    
    loop {
        tokio::select! {
            Some(()) = &mut local => {},
            Some(()) = &mut peer => {},
            else => break,
        }
    }
}

async fn local_socket_event(mut socket: SocketRuntimeLocal) -> Option<()> {
    match socket.receiver.recv().await.unwrap() {
        SocketEvent::Accept(responder, Some(timeout)) => {
            let result = time::timeout(timeout, accept(&socket))
                .await
                .unwrap_or_else(|error| Err(anyhow::Error::new(error)));
            responder.into_inner()
                .send(result)
                .ok()
        },
        SocketEvent::Accept(responder, None) => {
            let result = accept(&socket).await;
            responder.into_inner()
                .send(result)
                .ok()
        }
        SocketEvent::Close(application_error, reason) => {
            close(&socket, application_error, reason).await.unwrap();
            None
        },
        SocketEvent::Connect(responder, sock_addr, Some(timeout)) => {
            let result = time::timeout(timeout, connect(&mut socket, sock_addr))
                .await
                .unwrap_or_else(|error| Err(anyhow::Error::new(error)));
            responder.into_inner()
                .send(result)
                .ok()
        },
        SocketEvent::Connect(responder, sock_addr, None) => {
            let result = connect(&mut socket, sock_addr).await;
            responder.into_inner()
                .send(result)
                .ok()
        },
        SocketEvent::Listen(responder, sock_addr) => {
            let (sender, new_socket) = SocketRuntimeLocal::new(socket.runtime.clone());
            listen(new_socket, sock_addr).await.unwrap();
            responder.into_inner()
                .send(Ok(sender.into()))
                .ok()
        },
        SocketEvent::OpenBiStream(responder) => {
            let stream = open_bi_stream(&mut socket).await;
            responder.into_inner()
                .send(stream)
                .ok()
        },
        SocketEvent::OpenUniStream(responder) => {
            let stream = open_uni_stream(&mut socket).await;
            responder.into_inner()
                .send(stream)
                .ok()
        },
    }
}

async fn peer_socket_event(socket: SocketRuntime) -> Option<()> {
    let uni_streams = peer_uni_stream(socket.state.peer.uni_streams.clone());
    let bi_streams = peer_bi_stream(socket.state.peer.bi_streams.clone());
    tokio::pin!(uni_streams, bi_streams);
    let new_stream = tokio::select! {
        Some(stream) = &mut uni_streams => Some(stream),
        Some(stream) = &mut bi_streams => Some(stream),
        else => None,
    };
    new_stream.and_then(announce_stream)
}

async fn peer_uni_stream(uni_streams: Arc<Mutex<Option<IncomingUniStreams>>>) -> Option<StreamRuntime> {
    let lock = uni_streams.lock_owned().await;
    let stream = (*lock).as_mut().map(|x| x.next().await.unwrap())??;
    stream
}

async fn peer_bi_stream(bi_streams: Arc<Mutex<Option<IncomingBiStreams>>>) -> Option<StreamRuntime> {
    unimplemented!();
}

async fn accept(socket: &SocketRuntimeLocal) -> Result<Socket> {
    unimplemented!();
}

async fn connect(socket: &mut SocketRuntimeLocal, sock_addr: SocketAddr) -> Result<()> {
    unimplemented!();
}

async fn close(socket: &SocketRuntimeLocal, application_error: ApplicationError, reason: Option<Vec<u8>>) -> Result<()> {
    unimplemented!();
}

async fn listen(socket: SocketRuntimeLocal, sock_addr: SocketAddr) -> Result<()> {
    unimplemented!();
}

async fn open_bi_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    unimplemented!();
}

async fn open_uni_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    unimplemented!();
}

fn announce_stream(stream: StreamRuntime) -> Option<()> {
    unimplemented!();
}

async fn run_stream(stream: StreamRuntimeLocal) {
    let peer = peer_stream_event(stream.runtime.clone());
    let local = local_stream_event(stream);
    tokio::pin!(local, peer);
    
    loop {
        tokio::select! {
            Some(()) = &mut local => {},
            Some(()) = &mut peer => {},
            else => break
        }
    }
}

async fn local_stream_event(mut stream: StreamRuntimeLocal) -> Option<()> {
    match stream.receiver.recv().await.unwrap() {
        StreamEvent::CloseStream(application_error) => {
            close_stream(&stream, application_error).await.unwrap();
            None
        },
        StreamEvent::Read(responder, buffer, Some(timeout)) => {
            let result = time::timeout(timeout, read(&mut stream, buffer))
                .await
                .unwrap_or_else(|error| Err(anyhow::Error::new(error)));
            responder.into_inner()
                .send(result)
                .ok()
        },
        StreamEvent::Read(responder, buffer, None) => {
            let result = read(&mut stream, buffer).await;
            responder.into_inner()
                .send(result)
                .ok()
        },
        StreamEvent::Write(responder, buffer) => {
            None
        },
    }
}

async fn peer_stream_event(mut stream: StreamRuntime) -> Option<()> {
    unimplemented!();
}

async fn close_stream(stream: &StreamRuntimeLocal, application_error: ApplicationError) -> Result<()> {
    unimplemented!();
}

async fn read(stream: &mut StreamRuntimeLocal, buffer: Arc<Mutex<Vec<u8>>>) -> Result<u64> {
    unimplemented!();
}

async fn write_stream(stream: &mut StreamRuntimeLocal, buffer: Vec<u8>) -> Result<()> {
    unimplemented!();
}
