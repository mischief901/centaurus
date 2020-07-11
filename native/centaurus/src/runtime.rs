//! Provides traits and types for working with the Tokio runtime.
use crate::config::{ Configs };
use crate::conn::{ NewSocket, Socket, Stream };
use crate::error::{ ApplicationError };
use crate::state::{ SocketState, StreamState };
use crate::interface::types::{ SocketType };

use anyhow::{ Context, Result };

use either::{ Either };

use quinn::{
    EndpointBuilder,
    IncomingBiStreams,
    IncomingUniStreams,
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
    thread::{ JoinHandle },
    time::{ Duration },
};

type Responder<T> = Mutex<Sender<T>>;

/// Starts a new tokio runtime.
/// The runtime is a threaded pool named "centaurus-pool".
fn new_pool() -> runtime::Runtime {
    trace!("Opening new thread pool.");
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
        trace!("Building new socket runtime.");
        let bind_addr = configs
            .socket_config
            .read()
            .await
            .address()
            .context("Unable to get bind address.")?;
        trace!("Binding new socket.");
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
        trace!("Opening new socket channel to runtime.");
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
        trace!("Opening socket channel to runtime.");
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
        trace!("Opening new stream.");
        StreamRuntime {
            configs,
            state,
        }
    }
}

impl StreamRuntimeLocal {
    fn new(runtime: StreamRuntime) -> (AsyncSender<StreamEvent>, Self) {
        trace!("Opening new stream runtime.");
        let (sender, receiver) = unbounded_channel();
        let new_socket = Self {
            receiver,
            runtime,
        };
        (sender, new_socket)
    }
}

pub fn handle() -> Result<Either<AsyncSender<Event>, JoinHandle<()>>> {
    static mut SENDER : Option<AsyncSender<Event>> = None;
    static mut RUNTIME : Option<JoinHandle<()>> = None;
    static INIT : Once = Once::new();
    let response = unsafe {
        INIT.call_once(|| {
            trace!("Initializing runtime.");
            let (sender, receiver) : (AsyncSender<Event>,
                                      AsyncReceiver<Event>) = unbounded_channel();
            SENDER = Some(sender);
            RUNTIME = Some(std::thread::spawn(move || {
                trace!("Runtime thread started.");
                run(receiver);
            }));
        });
        // This will only match once.
        if let Some(runtime) = RUNTIME.take() {
            trace!("Got runtime.");
            Either::Right(runtime)
        } else {
            trace!("Got handle to runtime.");
            Either::Left(SENDER.as_ref().unwrap().clone())
        }
    };
    Ok(response)
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
    trace!("Opened new thread pooling.");
    rt.block_on(async {
        let result = tokio::join!{
            tokio::spawn(async move {
                trace!("Starting runtime loop.");
                loop {
                    match pool.recv().await.unwrap() {
                        Event::OpenSocket(responder, conn_type, configs, state) => {
                            trace!("Runtime received open socket event.");
                            let runtime = SocketRuntime::new(configs, state).await;
                            match runtime {
                                Ok(runtime) => {
                                    trace!("Opened socket on runtime.");
                                    let (sender, socket) = NewSocketRuntimeLocal::new(runtime);
                                    // Send the socket's sender to the synchronous side.
                                    trace!("Sending socket handle back.");
                                    responder.into_inner()
                                        .send(Ok(sender.into()))
                                        .unwrap();
                                    // Spawn a new task to handle the socket.
                                    tokio::spawn(async move {
                                        trace!("Running new socket.");
                                        run_new_socket(socket).await;
                                    });
                                },
                                Err(error) => {
                                    trace!("Error opening new socket: {}", error);
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
    trace!("Entering new socket runtime loop.");
    loop {
        match socket.receiver.recv().await {
            Some(NewSocketEvent::Accept(responder, Some(timeout))) => {
                trace!("New socket runtime received accept event.");
                let result = time::timeout(timeout, accept(&socket))
                    .await
                    .unwrap_or_else(|_error| Err(anyhow::anyhow!("Accept Timeout.")));
                responder.into_inner()
                    .send(result)
                    .ok();
            },
            Some(NewSocketEvent::Accept(responder, None)) => {
                trace!("New socket runtime received accept event.");
                let result = accept(&socket).await;
                responder.into_inner()
                    .send(result)
                    .ok();
            }
            Some(NewSocketEvent::Connect(responder, sock_addr, Some(timeout))) => {
                trace!("New socket runtime received connect event.");
                let result = time::timeout(timeout, connect(&mut socket, sock_addr))
                    .await
                    .unwrap_or_else(|_error| Err(anyhow::anyhow!("Connect Timeout.")));
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
            Some(NewSocketEvent::Connect(responder, sock_addr, None)) => {
                trace!("New socket runtime received connect event.");
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
            Some(NewSocketEvent::Close(application_error, reason)) => {
                trace!("New socket runtime received close event.");
                close_new(socket, application_error, reason).await.ok();
                break
            },
            None => {
                trace!("New socket runtime handle closed.");
                break
            },
        };
    }
}

async fn run_socket(socket: SocketRuntimeLocal) {
    trace!("Entering run socket loop.");
    let peer = peer_socket_event(socket.runtime.clone());
    let local = local_socket_event(socket);
    tokio::pin!(local, peer);
    
    loop {
        tokio::select! {
            Some(()) = &mut local => {
                trace!("Local event completed successfully.");
            },
            Ok(()) = &mut peer => {
                trace!("Peer event completed successfully.");
            },
            else => break,
        }
    };
    trace!("Socket runtime failure.");
}

async fn local_socket_event(mut socket: SocketRuntimeLocal) -> Option<()> {
    trace!("In local socket event loop.");
    match socket.receiver.recv().await.unwrap() {
        SocketEvent::Close(application_error, reason) => {
            trace!("Received local close socket event.");
            close(&socket, application_error, reason).await.ok();
            None
        },
        SocketEvent::OpenBiStream(responder) => {
            trace!("Received open local bi stream event.");
            let stream = open_bi_stream(&mut socket).await;
            responder.into_inner()
                .send(stream)
                .ok()
        },
        SocketEvent::OpenUniStream(responder) => {
            trace!("Received open local uni stream event.");
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
    trace!("In peer socket event loop.");
    let stream = tokio::select! {
        Some(state) = &mut uni_streams => state.map(|state| {
            trace!("Opening new peer uni stream.");
            StreamRuntime::new(state, socket.configs.clone())
        }),
        Some(state) = &mut bi_streams => state.map(|state| {
            trace!("Opening new peer bi stream.");
            StreamRuntime::new(state, socket.configs.clone())
        }),
        else => Err(anyhow::anyhow!("Incoming Streams Closed by peer.")),
    };
    trace!("Announcing new stream.");
    announce_stream(stream?).await
}

async fn peer_uni_stream(uni_streams: Option<Arc<Mutex<IncomingUniStreams>>>) -> Option<Result<StreamState>> {
    trace!("Locking incoming uni streams.");
    let mut lock = uni_streams?
        .lock_owned()
        .await;
    trace!("Waiting for incoming uni streams.");
    let stream = (*lock)
        .next()
        .await
        .map(|stream| stream.map(|s| s.into())
             .map_err(|e| e.into()));
    trace!("Received new uni stream.");
    stream
}

async fn peer_bi_stream(bi_streams: Option<Arc<Mutex<IncomingBiStreams>>>) -> Option<Result<StreamState>> {
    trace!("Locking incoming bi streams.");
    let mut lock = bi_streams?
        .lock_owned()
        .await;
    trace!("Waiting for incoming bi streams.");
    let stream = (*lock)
        .next()
        .await
        .map(|stream| stream.map(|s| s.into())
             .map_err(|e| e.into()));
    trace!("Received new bi stream.");
    stream
}

async fn accept(socket: &NewSocketRuntimeLocal) -> Result<Socket> {
    trace!("Accepting new socket.");
    let new_socket_state : SocketState = {
        (*socket
         .runtime
         .state
         .local
         .as_ref()
         .map(|x| {
             trace!("Waiting for socket state lock.");
             x
         })
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
            .incoming
            .as_mut()
            .map(|x| {
                trace!("Have socket state lock. Waiting for incoming socket.");
                x
            })
            .ok_or_else(|| anyhow::anyhow!("Incoming Socket closed."))?
            .next()
            .await
            .map(|x| {
                trace!("Received new socket request. Trying to connect.");
                x
            })
            .ok_or_else(|| anyhow::anyhow!("Incoming Socket closed."))?
            .await
            .map(|s| {
                trace!("Established new connection.");
                s.into()
            })?
    };

    trace!("Opening socket runtime for new socket.");
    let (sender, socket_local) = {
        let new_socket = SocketRuntime {
            configs: socket.runtime.configs.clone(),
            state: new_socket_state.clone(),
        };
        SocketRuntimeLocal::new(new_socket)
    };
    
    tokio::spawn(async move {
        trace!("Running new socket runtime.");
        run_socket(socket_local).await;
    });
    Ok(sender.into())
}

async fn connect(socket: &mut NewSocketRuntimeLocal, sock_addr: SocketAddr) -> Result<(AsyncSender<SocketEvent>, SocketRuntimeLocal)> {
    trace!("Connecting new socket.");
    trace!("Waiting to lock socket config.");
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
         .map(|x| {
             trace!("Waiting for state lock.");
             x
         })
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
            .endpoint
            .as_mut()
            .map(|x| {
                trace!("Have state lock. Trying to establish connection.");
                x
            })
            .unwrap()
            .connect(&sock_addr, &server_name)?
            .await?
            .into()
    };
    trace!("Connection established.");
    let socket = SocketRuntime {
        configs: socket.runtime.configs.clone(),
        state: new_socket_state,
    };
    Ok(SocketRuntimeLocal::new(socket))
}

// TODO: Find a way to combine these two close functions.
async fn close_new(socket: NewSocketRuntimeLocal, application_error: ApplicationError, reason: Option<String>) -> Result<()> {
    trace!("Closing new socket runtime.");
    let reason = reason.unwrap_or_else(|| "".to_string());
    (*socket
     .runtime
     .state
     .local
     .as_ref()
     .map(|x| {
         trace!("Waiting for local state lock.");
         x
     })
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .map(|x| {
            trace!("Closing connection.");
            x
        })
        .unwrap()
        .close(application_error.into(), reason.as_bytes());
            
    Ok(())
}

async fn close(socket: &SocketRuntimeLocal, application_error: ApplicationError, reason: Option<String>) -> Result<()> {
    trace!("Closing socket runtime.");
    let reason = reason.unwrap_or_else(|| "".to_string());
    (*socket
     .runtime
     .state
     .local
     .as_ref()
     .map(|x| {
         trace!("Waiting for local state lock.");
         x
     })
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .connection
        .as_mut()
        .map(|x| {
            trace!("Closing connection.");
            x
        })
        .unwrap()
        .close(application_error.into(), reason.as_bytes());
            
    Ok(())
}

async fn open_bi_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    trace!("Opening new bi stream.");
    let state : StreamState =
        (*socket
         .runtime
         .state
         .local
         .as_ref()
         .map(|x| {
             trace!("Waiting for local state lock.");
             x
         })
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
        .connection
        .as_mut()
        .unwrap()
        .open_bi()
        .into();
    trace!("Opened new bi stream.");
    let runtime = StreamRuntime::new(state, socket.runtime.configs.clone());
    let (sender, stream) = StreamRuntimeLocal::new(runtime);
    
    tokio::spawn(async move {
        trace!("Running new bi stream runtime.");
        run_stream(stream).await;
    });
    
    Ok(sender.into())
}

async fn open_uni_stream(socket: &mut SocketRuntimeLocal) -> Result<Stream> {
    trace!("Opening new uni stram.");
    let state : StreamState =
        (*socket
         .runtime
         .state
         .local
         .as_ref()
         .map(|x| {
             trace!("Waiting for local state lock.");
             x
         })
         .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
         .lock()
         .await)
        .connection
        .as_mut()
        .unwrap()
        .open_uni()
        .into();
    trace!("Opened new uni stream.");
    let runtime = StreamRuntime::new(state, socket.runtime.configs.clone());
    let (sender, stream) = StreamRuntimeLocal::new(runtime);

    tokio::spawn(async move {
        trace!("Running new uni stream runtime.");
        run_stream(stream).await;
    });
    
    Ok(sender.into())
}

async fn announce_stream(stream: StreamRuntime) -> Result<()> {
    trace!("Announing new stream.");
    let (sender, stream_local) = StreamRuntimeLocal::new(stream.clone());
    tokio::spawn(async move {
        trace!("Running new stream runtime.");
        run_stream(stream_local).await;
    });
    trace!("Waiting for socket config lock.");
    let config_lock = stream.configs.socket_config.read().await;
    // I don't like this. Should try to find something better.
    // Maybe involving a boxed fn for creating the stream.
    trace!("Sending new stream announcement.");
    (*config_lock).send(crate::interface::types::Stream::from(Stream(sender)))?;
    trace!("New stream announcement successful.");
    Ok(())
}

// TODO: Add active read capability. Similar to peer_socket_event above.
async fn run_stream(stream: StreamRuntimeLocal) {
    let local = local_stream_event(stream);
    tokio::pin!(local);
    trace!("Entering local stream runtime loop.");
    loop {
        tokio::select! {
            Some(()) = &mut local => {},
            else => break
        }
    }
}

async fn local_stream_event(mut stream: StreamRuntimeLocal) -> Option<()> {
    trace!("Local stream event runtime.");
    match stream.receiver.recv().await.unwrap() {
        StreamEvent::CloseStream(application_error) => {
            trace!("Received stream close event.");
            close_stream(&stream, application_error).await.unwrap();
            None
        },
        StreamEvent::Read(responder, buffer, Some(timeout)) => {
            trace!("Received stream read event.");
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
            trace!("Received stream read event.");
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
            trace!("Received stream write event.");
            let result = write(&mut stream, buffer).await;
            responder.into_inner()
                .send(result)
                .ok()
        },
    }
}

async fn close_stream(stream: &StreamRuntimeLocal, application_error: ApplicationError) -> Result<()> {
    trace!("Closing stream.");
    (*stream
     .runtime
     .state
     .local
     .as_ref()
     .map(|x| {
         trace!("Waiting for local state lock.");
         x
     })
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .recv
        .as_mut()
        .unwrap()
        .stop(application_error.into())
        .map_err(|_err| anyhow::anyhow!("Error Closing Stream."))
        .map(|x| {
            trace!("Stream closed.");
            x
        })
}

async fn read(stream: &mut StreamRuntimeLocal, buffer: Arc<Mutex<Vec<u8>>>) -> Option<Result<usize>> {
    trace!("Reading from stream.");
    trace!("Waiting for buffer lock.");
    let mut buf = buffer.lock().await;
    (*stream
     .runtime
     .state
     .local
     .as_ref()
     .map(|x| {
         trace!("Waiting for local stream state lock.");
         x
     })?
     .lock()
     .await)
        .recv
        .as_mut()
        .map(|x| {
            trace!("Reading into buffer.");
            x
        })
        .unwrap()
        .read(&mut (*buf.as_mut_slice()))
        .await
        .map_err(|_err| anyhow::anyhow!("Error Reading from Stream."))
        .transpose()
}

async fn write(stream: &mut StreamRuntimeLocal, mut buffer: Vec<u8>) -> Result<()> {
    trace!("Writing to stream.");
    (*stream
     .runtime
     .state
     .local
     .as_ref()
     .map(|x| {
         trace!("Waiting for local stream state lock.");
         x
     })
     .ok_or_else(|| anyhow::anyhow!("Local State Inaccessible."))?
     .lock()
     .await)
        .send
        .as_mut()
        .map(|x| {
            trace!("Writing to stream.");
            x
        })
        .unwrap()
        .write_all(buffer.as_mut_slice())
        .await
        .context("Error Writing to Stream.")
}
