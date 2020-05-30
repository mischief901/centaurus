//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs };
use crate::conn::{ NewSocket, Socket, Stream };
use crate::error::{ ApplicationError };
use crate::state::{ SocketState, StreamState };
use crate::interface::types::{ SocketType };

use anyhow::{ Context, Result };

use quinn::{
    EndpointBuilder,
    IncomingBiStreams,
    IncomingUniStreams,
    OpenBi,
    OpenUni
};

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
    fmt,
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

impl SocketRuntime {
    async fn new(configs: Configs, builder: EndpointBuilder) -> Result<Self> {
        let bind_addr = configs
            .socket_config
            .read()
            .await
            .address()
            .context("Unable to get bind address.")?;
        builder.bind(&bind_addr)
            .map(|state| {
                Self {
                    configs,
                    state: state.into(),
                }
            })
            .context("Error binding socket.")
    }
}

#[derive(Debug)]
struct NewSocketRuntimeLocal {
    receiver: AsyncReceiver<NewSocketEvent>,
    runtime: SocketRuntime,
}

impl NewSocketRuntimeLocal {
    fn new(runtime: SocketRuntime) -> (AsyncSender<NewSocketEvent>, Self) {
        let (sender, receiver) = unbounded_channel();
        let new_socket = Self {
            receiver,
            runtime,
        };
        (sender, new_socket)
    }
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

impl StreamRuntime {
    fn new(state: StreamState, configs: Configs) -> StreamRuntime {
        StreamRuntime {
            configs,
            state,
        }
    }
}

impl StreamRuntimeLocal {
    fn new(runtime: StreamRuntime) -> (AsyncSender<StreamEvent>, Self) {
        let (sender, receiver) = unbounded_channel();
        let new_socket = Self {
            receiver,
            runtime,
        };
        (sender, new_socket)
    }
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

pub enum Event {
    OpenSocket(Responder<Result<NewSocket>>, SocketType, Configs, EndpointBuilder),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Endpoint Builder.")
    }
}

#[derive(Debug)]
// Events a non-connected socket knows how to handle. Spawns a new socket connection.
pub enum NewSocketEvent {
    Accept(Responder<Result<Socket>>, Option<Duration>),
    Connect(Responder<Result<Socket>>, SocketAddr, Option<Duration>),
    Close(ApplicationError, Option<String>),
}

#[derive(Debug)]
// Events a connected socket knows how to handle.
pub enum SocketEvent {
    Close(ApplicationError, Option<String>),
    OpenBiStream(Responder<Result<Stream>>),
    OpenUniStream(Responder<Result<Stream>>),
}

#[derive(Debug)]
// Events the stream knows how to handle.
pub enum StreamEvent {
    CloseStream(ApplicationError),
    Read(Responder<Result<usize>>, Arc<Mutex<Vec<u8>>>, Option<Duration>),
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
                            let runtime = SocketRuntime::new(configs, state).await;
                            match runtime {
                                Ok(runtime) => {
                                    let (sender, socket) = NewSocketRuntimeLocal::new(runtime);
                                    // Send the socket's sender to the synchronous side.
                                    responder.into_inner()
                                        .send(Ok(sender.into()))
                                        .unwrap();
                                    // Spawn a new task to handle the socket.
                                    tokio::spawn(async move {
                                        run_new_socket(socket).await;
                                    });
                                },
                                Err(error) => {
                                    responder.into_inner()
                                        .send(Err(error))
                                        .unwrap();
                                }
                            }
                        }
                    }
                }
            })
        };
        result.0.unwrap();
    });
}

// These actions create a new socket on the runtime.
async fn run_new_socket(mut socket: NewSocketRuntimeLocal) {
    loop {
        match socket.receiver.recv().await.unwrap() {
            NewSocketEvent::Accept(responder, Some(timeout)) => {
                let result = time::timeout(timeout, accept(&socket))
                    .await
                    .unwrap_or_else(|error| Err(anyhow::Error::new(error)));
                responder.into_inner()
                    .send(result)
                    .ok();
            },
            NewSocketEvent::Accept(responder, None) => {
                let result = accept(&socket).await;
                responder.into_inner()
                    .send(result)
                    .ok();
            }
            NewSocketEvent::Connect(responder, sock_addr, Some(timeout)) => {
                let result = time::timeout(timeout, connect(&mut socket, sock_addr))
                    .await
                    .unwrap_or_else(|error| Err(anyhow::Error::new(error)));
                match result {
                    Ok((sender, socket)) => {
                        responder.into_inner()
                            .send(Ok(sender.into()))
                            .ok();
                        run_socket(socket).await;
                    },
                    Err(err) => {
                        responder.into_inner()
                            .send(Err(err))
                            .ok();
                        break
                    }
                }
            },
            NewSocketEvent::Connect(responder, sock_addr, None) => {
                let result = connect(&mut socket, sock_addr).await;
                match result {
                    Ok((sender, socket)) => {
                        responder.into_inner()
                            .send(Ok(sender.into()))
                            .ok();
                        run_socket(socket).await;
                    },
                    Err(err) => {
                        responder.into_inner()
                            .send(Err(err))
                            .ok();
                        break
                    }
                }
            },
            NewSocketEvent::Close(application_error, reason) => {
                close_new(socket, application_error, reason).await.ok();
                break
            },
        };
    }
}

async fn run_socket(socket: SocketRuntimeLocal) {
    let peer = peer_socket_event(socket.runtime.clone());
    let local = local_socket_event(socket);
    tokio::pin!(local, peer);
    
    loop {
        tokio::select! {
            Some(()) = &mut local => {},
            Ok(()) = &mut peer => {},
            else => break,
        }
    }
}

async fn local_socket_event(mut socket: SocketRuntimeLocal) -> Option<()> {
    match socket.receiver.recv().await.unwrap() {
        SocketEvent::Close(application_error, reason) => {
            close(&socket, application_error, reason).await.ok();
            None
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

async fn peer_socket_event(socket: SocketRuntime) -> Result<()> {
    let unis = socket
        .state
        .peer
        .uni_streams
        .clone();
    let bis = socket
        .state
        .peer
        .bi_streams
        .clone();
    let uni_streams = peer_uni_stream(unis);
    let bi_streams = peer_bi_stream(bis);

    tokio::pin!(uni_streams, bi_streams);
    
    let stream = tokio::select! {
        Some(state) = &mut uni_streams => state.map(|state| StreamRuntime::new(state, socket.configs.clone())),
        Some(state) = &mut bi_streams => state.map(|state| StreamRuntime::new(state, socket.configs.clone())),
        else => Err(anyhow::anyhow!("Incoming Streams Closed by peer.")),
    };
    announce_stream(stream?).await
}

async fn peer_uni_stream(uni_streams: Option<Arc<Mutex<IncomingUniStreams>>>) -> Option<Result<StreamState>> {
    let mut lock = uni_streams?
        .lock_owned()
        .await;
    let stream = (*lock)
        .next()
        .await
        .map(|stream| stream.map(|s| s.into())
             .map_err(|e| e.into()));
    stream
}

async fn peer_bi_stream(bi_streams: Option<Arc<Mutex<IncomingBiStreams>>>) -> Option<Result<StreamState>> {
    let mut lock = bi_streams?
        .lock_owned()
        .await;
    let stream = (*lock)
        .next()
        .await
        .map(|stream| stream.map(|s| s.into())
             .map_err(|e| e.into()));
    stream
}

async fn accept(socket: &NewSocketRuntimeLocal) -> Result<Socket> {
    let new_socket_state : SocketState = {
        (*socket
         .runtime
         .state
         .local
         .as_ref()
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
            .incoming
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Incoming Streams closed."))?
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Incoming Streams closed."))?
            .await
            .map(|s| s.into())?
    };
    
    let (sender, socket_local) = {
        let new_socket = SocketRuntime {
            configs: socket.runtime.configs.clone(),
            state: new_socket_state.clone(),
        };
        SocketRuntimeLocal::new(new_socket)
    };
    
    tokio::spawn(async move {
        run_socket(socket_local).await;
    });
    Ok(sender.into())
}

async fn connect(socket: &mut NewSocketRuntimeLocal, sock_addr: SocketAddr) -> Result<(AsyncSender<SocketEvent>, SocketRuntimeLocal)> {
    let server_name = {
        (*socket
         .runtime
         .configs
         .socket_config
         .read()
         .await)
            .server_name()?
    };
    let new_socket_state : SocketState = {
        (*socket
         .runtime
         .state
         .local
         .as_ref()
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
            .endpoint
            .as_mut()
            .unwrap()
            .connect(&sock_addr, &server_name)?
            .await?
            .into()
    };
    let socket = SocketRuntime {
        configs: socket.runtime.configs.clone(),
        state: new_socket_state,
    };
    Ok(SocketRuntimeLocal::new(socket))
}

// TODO: Find a way to combine these two close functions.
async fn close_new(socket: NewSocketRuntimeLocal, application_error: ApplicationError, reason: Option<String>) -> Result<()> {
    let reason = reason.unwrap_or_else(|| "".to_string());
    (*socket
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .unwrap()
        .close(application_error.into(), reason.as_bytes());
            
    Ok(())
}

async fn close(socket: &SocketRuntimeLocal, application_error: ApplicationError, reason: Option<String>) -> Result<()> {
    let reason = reason.unwrap_or_else(|| "".to_string());
    (*socket
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .unwrap()
        .close(application_error.into(), reason.as_bytes());
            
    Ok(())
}

async fn open_bi_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    let state : StreamState = (*socket
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .unwrap()
        .open_bi()
        .into();
    let runtime = StreamRuntime::new(state, socket.runtime.configs.clone());
    let (sender, stream) = StreamRuntimeLocal::new(runtime);

    tokio::spawn(async move {
        run_stream(stream).await;
    });
    
    Ok(sender.into())
}

async fn open_uni_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    let state : StreamState = (*socket
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .unwrap()
        .open_uni()
        .into();
    let runtime = StreamRuntime::new(state, socket.runtime.configs.clone());
    let (sender, stream) = StreamRuntimeLocal::new(runtime);

    tokio::spawn(async move {
        run_stream(stream).await;
    });
    
    Ok(sender.into())
}

async fn announce_stream(stream: StreamRuntime) -> Result<()> {
    let (sender, stream_local) = StreamRuntimeLocal::new(stream.clone());
    tokio::spawn(async move {
        run_stream(stream_local).await;
    });
    let config_lock = stream.configs.socket_config.read().await;
    // I don't like this. Should try to find something better.
    // Maybe involving a boxed fn for creating the stream.
    (*config_lock).send(crate::interface::types::Stream::from(Stream(sender)))?;
    Ok(())
}

// TODO: Add active read capability. Similar to peer_socket_event above.
async fn run_stream(stream: StreamRuntimeLocal) {
    let local = local_stream_event(stream);
    tokio::pin!(local);
    
    loop {
        tokio::select! {
            Some(()) = &mut local => {},
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
            if let Some(result) = time::timeout(timeout, read(&mut stream, buffer))
                .await
                .unwrap_or_else(|error| Some(Err(anyhow::Error::new(error)))) {
                    responder.into_inner()
                        .send(result)
                        .ok()
                } else {
                    responder.into_inner()
                        .send(Err(anyhow::anyhow!("Stream Closed.")))
                        .ok();
                    None
                }
        },
        StreamEvent::Read(responder, buffer, None) => {
            if let Some(result) = read(&mut stream, buffer).await {
                responder.into_inner()
                    .send(result)
                    .ok()
            } else {
                responder.into_inner()
                    .send(Err(anyhow::anyhow!("Stream Closed.")))
                    .ok();
                None
            }
        },
        StreamEvent::Write(responder, buffer) => {
            let result = write(&mut stream, buffer).await;
            responder.into_inner()
                .send(result)
                .ok()
        },
    }
}

async fn close_stream(stream: &StreamRuntimeLocal, application_error: ApplicationError) -> Result<()> {
    (*stream
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .recv
        .as_mut()
        .unwrap()
        .stop(application_error.into())
        .map_err(|_err| anyhow::anyhow!("Error Closing Stream."))
}

async fn read(stream: &mut StreamRuntimeLocal, buffer: Arc<Mutex<Vec<u8>>>) -> Option<Result<usize>> {
    let mut buf = buffer.lock().await;
    (*stream
     .runtime
     .state
     .local
     .as_ref()?
     .lock()
     .await)
        .recv
        .as_mut()
        .unwrap()
        .read(&mut (*buf.as_mut_slice()))
        .await
        .map_err(|_err| anyhow::anyhow!("Error Reading from Stream."))
        .transpose()
        
}

async fn write(stream: &mut StreamRuntimeLocal, mut buffer: Vec<u8>) -> Result<()> {
    (*stream
     .runtime
     .state
     .local
     .as_ref()
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .send
        .as_mut()
        .unwrap()
        .write_all(buffer.as_mut_slice())
        .await
        .context("Error Writing to Stream.")
}
