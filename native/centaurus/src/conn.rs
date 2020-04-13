/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::{ ApplicationError, Error };
use crate::config::{ Config, ConnType };
use crate::runtime::{ RunSocket, RunStream, Runtime };
use crate::state::{ State, StreamState };
use crate::notify::{ Notify };

use futures::{ StreamExt, pin_mut, select };
use futures::future::{ FutureExt };

use quinn::{
    ClientConfigBuilder,
    Endpoint,
    Incoming,
    NewConnection,
    OpenBi,
    OpenUni,
    ServerConfig,
    ServerConfigBuilder,
    TransportConfig
};

use tokio::runtime::{ Handle };
use tokio::task;
use tokio::task::{ JoinHandle };
use tokio::time;

use std::{
    default::{ Default },
    future::{ Future },
    net::{ SocketAddr },
    sync::{ Arc },
    time::{ Duration },
};

/// The Socket struct ties a block of `meta` data to a connection endpoint
#[derive(Clone)]
pub struct Socket<T : Clone + Config + Notify<S>, S : Clone + Default + Send + Sync> {
    pub meta: T,
    conn: Runtime,
    pub handle: Handle,
    pub conn_type: ConnType,
    pub stream_config: Option<S>,
    state: State,
}

impl <T : 'static + Clone + Config + Notify<S>, S : 'static + Clone + Default + Send + Sync> Socket <T, S> {
    pub fn new(meta: T, conn_type: ConnType, stream_config: Option<S>) -> Result<Self, Error> {
        let mut endpoint = Endpoint::builder();
        let sock_addr = meta.address()?;
        let conn = Runtime::new();
        let handle = conn.handle();
        match conn_type {
            ConnType::Client => {
                let certs = meta.certs()?;
                let mut client = ClientConfigBuilder::default();
                client.add_certificate_authority(certs)?;
                endpoint.default_client_config(client.build());
                let (endpoint, _incoming) = handle.enter(move || {
                    endpoint.bind(&sock_addr)
                })?;
                Ok(Socket{
                    meta,
                    conn,
                    handle,
                    conn_type,
                    stream_config,
                    state: endpoint.into(),
                })
            },
            ConnType::Server => {
                let private_key = meta.private_key()?;
                let mut transport_config = TransportConfig::default();
                transport_config.stream_window_uni(0);
                let mut server_config = ServerConfig::default();
                server_config.transport = Arc::new(transport_config);
                let cert_chain = meta.cert_chain()?;
                let mut server = ServerConfigBuilder::new(server_config);
                server.certificate(cert_chain, private_key)?;
                endpoint.listen(server.build());
                let (_endpoint, incoming) = handle.enter(move || {
                    endpoint.bind(&sock_addr)
                })?;
                Ok(Socket{
                    meta,
                    conn,
                    handle,
                    conn_type,
                    stream_config,
                    state: incoming.into(),
                })
            },
        }
    }

    pub async fn run_watchers(&self) -> Result<JoinHandle<()>, Error> {
        let watcher = self.clone();
        let watchers = self.conn.spawn(async {
            let watcher = watcher;
            let incoming_uni_streams = watcher.uni_watcher().fuse();
            let incoming_bi_streams = watcher.bi_watcher().fuse();
            pin_mut!(incoming_uni_streams, incoming_bi_streams);
            loop {
                select!{
                    stream = incoming_uni_streams => watcher.meta.peer_uni_stream(stream.unwrap()).unwrap(),
                    stream = incoming_bi_streams => watcher.meta.peer_bi_stream(stream.unwrap()).unwrap(),
                }
            };
        });
        Ok(watchers)
    }

    pub async fn uni_watcher(&self) -> Result<Stream<S>, Error> {
        unimplemented!();
    }

    pub async fn bi_watcher(&self) -> Result<Stream<S>, Error> {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct Stream<T : Default> {
    pub meta: Option<T>,
    conn: Runtime,
    pub handle: Handle,
    stream_state: StreamState,
}

impl <T : Default> Stream <T> {
    pub fn new(meta: Option<T>, conn: Runtime, handle: Handle, stream_state: StreamState) -> Self {
        Self {
            meta,
            conn,
            handle,
            stream_state,
        }
    }
}


impl<T : 'static + Clone + Config + Send + Sync + Notify<S>, S : 'static + Clone + Default + Send + Sync> RunSocket<Stream<S>> for Socket<T, S> {
    type Stream = Stream<S>;

    fn run_socket(&self) -> Result<(), Error> {
        self.run_watchers();
        Ok(())
    }
    
    fn accept(&self, timeout: Option<u64>) -> Result<Self, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let state = self.state.clone();
        self.conn.block_on(async move {
            // The first await is for connection coming in.
            // The second await is for setting up the connection.
            let future = state
                .lock()
                .incoming()
                .as_mut()
                .map(|incoming| incoming.next())
                .unwrap()
                .await
                .unwrap();
            let new_state = if let Some(duration) = timeout {
                time::timeout(duration, future)
                    .await
                    .unwrap()
                    .unwrap()
            } else {
                future.await
                    .unwrap()
            };
            Ok(Socket {
                conn: self.conn.clone(),
                handle: self.handle.clone(),
                state: new_state.into(),
                stream_config: self.stream_config.clone(),
                conn_type: self.conn_type.clone(),
                meta: self.meta.clone(),
            })
        })
    }
    
    fn connect(&self, address: SocketAddr, timeout: Option<u64>) -> Result<(), Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let server_name = self.meta.server_name()?;
        let state = self.state.clone();
        self.conn.block_on(async move {
            let future = state.lock()
                .endpoint()
                .as_mut()
                .map(|endpoint| endpoint.connect(&address, &server_name))
                .unwrap()
                .unwrap();
            let new_conn = if let Some(duration) = timeout {
                time::timeout(duration, future)
                    .await
                    .unwrap()
                    .unwrap()
            } else {
                future.await
                    .unwrap()
            };
            state.replace(new_conn);
        });
        self.run_socket()
    }
 
    fn new_uni_stream(&self) -> Result<Self::Stream, Error> {
        let stream_future = self.state
            .lock()
            .connection()
            .as_mut()
            .map(|connection| connection.open_uni())
            .unwrap();
        Ok(Self::Stream::new_uni_stream(self.stream_config.clone(), stream_future))
    }

    fn new_bi_stream(&self) -> Result<Self::Stream, Error> {
        let stream_future = self.state
            .lock()
            .connection()
            .as_mut()
            .map(|connection| connection.open_bi())
            .unwrap();
        Ok(Self::Stream::new_bi_stream(self.stream_config.clone(), stream_future))
    }

    fn close(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
        unimplemented!();
    }
}

impl<T : Clone + Send + Sync + Default> RunStream<Stream<T>, T> for Stream<T> {
    fn run_stream(self) -> Result<(), Error> {
        unimplemented!();
    }
    
    fn new_uni_stream(default_config: Option<T>, stream_future: OpenUni) -> Self {
        unimplemented!();
/*        Stream { meta: default_config,
                 stream_state: stream_future.into(),
                 ..Default::default() } */
    }

    fn new_bi_stream(default_config: Option<T>, stream_future: OpenBi) -> Self {
        unimplemented!();
/*        Stream { meta: default_config,
                 stream_state: stream_future.into(),
                 ..Default::default() }*/
    }
    
    fn read(&self, buffer: &mut [u8], timeout: Option<u64>) -> Result<u64, Error> {
        unimplemented!();
    }

    fn write(&self, buffer: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    fn close_stream(&self, error_code: ApplicationError, reason: Vec<u8>) -> Result<(), Error> {
        unimplemented!();
    }   
}

