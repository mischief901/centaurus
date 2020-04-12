/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::{ ApplicationError, Error };
use crate::config::{ Config, ConnType };
use crate::runtime::{ RunSocket, RunStream, Runtime };
use crate::state::{ State, StreamState };

use futures::StreamExt;

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

use tokio::time;

use std::{
    default::{ Default },
    net::{ SocketAddr },
    sync::{ Arc },
    time::{ Duration },
};

/// The Socket struct ties a block of `meta` data to a connection endpoint
pub struct Socket<T : Clone + Config, S : Clone + Default + Send + Sync> {
    pub meta: T,
    pub conn: Runtime,
    pub conn_type: ConnType,
    pub stream_config: Option<S>,
    state: State,
}

impl <T : Clone + Config, S : Clone + Default + Send + Sync> Socket <T, S> {
    pub fn new(meta: T, conn_type: ConnType, stream_config: Option<S>) -> Result<Self, Error> {
        let mut endpoint = Endpoint::builder();
        let sock_addr = meta.address()?;
        let conn = Runtime::new();
        match conn_type {
            ConnType::Client => {
                let certs = meta.certs()?;
                let mut client = ClientConfigBuilder::default();
                client.add_certificate_authority(certs)?;
                endpoint.default_client_config(client.build());
                let (endpoint, _incoming) = conn.enter(move || {
                    endpoint.bind(&sock_addr)
                })?;
                Ok(Socket{
                    meta,
                    conn,
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
                let (_endpoint, incoming) = conn.enter(move || {
                    endpoint.bind(&sock_addr)
                })?;
                Ok(Socket{
                    meta,
                    conn,
                    conn_type,
                    stream_config,
                    state: incoming.into(),
                })
            },
        }
    }
}

pub struct Stream<T : Default> {
    pub meta: Option<T>,
    pub conn: Runtime,
    stream_state: StreamState,
}

impl <T : Default> Stream <T> {
    pub fn new(meta: Option<T>, conn: Runtime, stream_state: StreamState) -> Self {
        Self {
            meta,
            conn,
            stream_state,
        }
    }
}

impl<T : Default> Default for Stream<T> {
    fn default() -> Self {
        let meta = Some(T::default());
        let conn = Runtime::new();
        let stream_state = StreamState::default();
        Self {
            meta,
            conn,
            stream_state
        }
    }
}

impl<T : Clone + Config + Send + Sync, S : Clone + Default + Send + Sync> RunSocket<Stream<S>> for Socket<T, S> {
    type Stream = Stream<S>;

    fn run_socket(self) -> Result<(), Error> {
        unimplemented!();
    }
    
    fn accept(&mut self, timeout: Option<u64>) -> Result<Self, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let mut incoming = self.state
            .incoming()
            .unwrap();
        let handle = self.conn.enter(async move || {
            // The first await is for connection coming in.
            // The second await is for setting up the connection.
            let future = incoming.next()
                .await
                .unwrap();
            let result = if let Some(duration) = timeout {
                (time::timeout(duration, future)
                 .await
                 .unwrap()
                 .unwrap(),
                 incoming)
            } else {
                (future.await
                 .unwrap(),
                 incoming)
            };
            result
        });
        let (new_state, incoming) : (NewConnection, Incoming) = self.conn.block_on(handle);
        let new_conn = Socket {
            conn: Runtime::new(),
            state: new_state.into(),
            stream_config: self.stream_config.clone(),
            conn_type: self.conn_type.clone(),
            meta: self.meta.clone(),
        };
        self.state.replace(incoming);
        Ok(new_conn)
    }

    fn connect(&mut self, address: SocketAddr, timeout: Option<u64>) -> Result<(), Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let server_name = self.meta.server_name()?;
        let endpoint = self.state
            .endpoint()
            .unwrap();
        let connection_handle = self.conn.enter(async move || {
            if let Some(duration) = timeout {
                time::timeout(duration,
                              endpoint.connect(&address, &server_name)
                              .unwrap())
                    .await
                    .unwrap()
                    .unwrap()
            } else {
                endpoint.connect(&address, &server_name)
                    .unwrap()
                    .await
                    .unwrap()
            }
        });
        
        let new_conn : NewConnection = self.conn.block_on(connection_handle);
        self.state = new_conn.into();
        Ok(())
    }
    
    fn new_uni_stream(&self) -> Result<Self::Stream, Error> {
        let stream = self.state
            .connection()
            .as_ref()
            .unwrap()
            .open_uni();
        Ok(Self::Stream::new_uni_stream(self.stream_config.clone(), stream))
    }

    fn new_bi_stream(&self) -> Result<Self::Stream, Error> {
        let stream = self.state
            .connection()
            .as_ref()
            .unwrap()
            .open_bi();
        Ok(Self::Stream::new_bi_stream(self.stream_config.clone(), stream))
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

