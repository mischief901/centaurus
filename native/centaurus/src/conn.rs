/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::{ ApplicationError, Error };
use crate::config::{ Configs, SocketConfig, SocketType, StreamConfig, StreamType };
use crate::interface;
use crate::runtime::{ Event, SocketEvent, StreamEvent };
use crate::state::{ SocketState };

use futures::{ StreamExt, pin_mut, select };
use futures::future::{ FutureExt };

use quinn::{
    ClientConfigBuilder,
    Endpoint,
    ServerConfig,
    ServerConfigBuilder,
    TransportConfig
};

use tokio::sync::{
    Mutex,
    mpsc::{
        UnboundedSender as AsyncSender,
    },
};

use std::{
    default::{ Default },
    future::{ Future },
    net::{ SocketAddr },
    sync::{ Arc },
    sync::mpsc::{ channel },
    time::{ Duration },
};

#[derive(Clone)]
pub struct Config<S : SocketConfig, T : StreamConfig> {
    pub socket_config: S,
    pub stream_config: T,
}

pub struct Socket<S : SocketConfig, T : StreamConfig>(AsyncSender<SocketEvent<S, T>>);
pub struct Stream<T : StreamConfig>(AsyncSender<StreamEvent<T>>);

impl <S : SocketConfig, T : StreamConfig> Socket <S, T> {    
    pub fn open_socket(conn_type: SocketType, socket_config: S, stream_config: T) -> Result<Self, Error> {
        let (sender, receiver) = channel();
        let mut endpoint = Endpoint::builder();
        let sock_addr = socket_config.address()?;
        let configs = Configs {
            socket_config,
            stream_config,
        };
        let socket_handle = interface::handle()?;
        let state : SocketState = match conn_type {
            SocketType::Client => {
                let certs = socket_config.certs()?;
                let mut client = ClientConfigBuilder::default();
                client.add_certificate_authority(certs)?;
                endpoint.default_client_config(client.build());
                endpoint.into()
            },
            SocketType::Server => {
                let private_key = socket_config.private_key()?;
                let mut transport_config = TransportConfig::default();
                transport_config.stream_window_uni(0);
                let mut server_config = ServerConfig::default();
                server_config.transport = Arc::new(transport_config);
                let cert_chain = socket_config.cert_chain()?;
                let mut server = ServerConfigBuilder::new(server_config);
                server.certificate(cert_chain, private_key)?;
                endpoint.listen(server.build());
                endpoint.into()
            },
        };
        let event = Event::OpenSocket(sender, conn_type, configs, state);
        socket_handle.send(event)?;
        receiver.recv()
    }

    fn listen(&self, address: SocketAddr, timeout: Option<u64>) -> Result<(), Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        self.send(SocketEvent::Listen(sender, address, timeout))?;
        receiver.recv()
    }
    
    fn accept(&self, timeout: Option<u64>) -> Result<Self, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        self.send(SocketEvent::Accept(sender, timeout))?;
        receiver.recv()
    }
/*        let handle = self.conn.enter(async move || {
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
        Ok(new_conn) */

    fn connect(&self, address: SocketAddr, timeout: Option<u64>) -> Result<(), Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        let event = SocketEvent::Connect(sender, address, timeout);
        self.send(event)?;
        receiver.recv()
    }
     /*   
        let connection_handle = self.conn.enter(async move || {
            if let Some(duration) = timeout {
                time::timeout(duration,
                              endpoint.connect(&address, &server_name)
                              .unwrap())
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
     */
    
    fn new_uni_stream(&self) -> Result<Stream<T>, Error> {
        let (sender, receiver) = channel();
        let event = SocketEvent::OpenUniStream(sender);
        self.send(event)?;
        receiver.recv()
    }
/*      let stream = self.state
            .connection()
            .as_mut()
            .map(|connection| connection.open_uni())
            .unwrap();
        Ok(Self::Stream::new_uni_stream(self.stream_config.clone(), stream_future))
    }
*/
    fn new_bi_stream(&self) -> Result<Stream<T>, Error> {
        let (sender, receiver) = channel();
        let event = SocketEvent::OpenBiStream(sender);
        self.send(event)?;
        receiver.recv()
    }
/*        let stream = self.state
            .connection()
            .as_mut()
            .map(|connection| connection.open_bi())
            .unwrap();
        Ok(Self::Stream::new_bi_stream(self.stream_config.clone(), stream_future))
    }
*/
    fn close(self, error_code: ApplicationError, reason: Vec<u8>) {
        let event = SocketEvent::Close(error_code, reason);
        self.send(event).unwrap();
    }
}

impl<T : StreamConfig> Stream<T> {
    fn read(&self, buffer: Vec<u8>, timeout: Option<u64>) -> Result<u64, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let safe_buffer = Arc::new(Mutex::new(buffer));
        let (sender, receiver) = channel();
        let event = StreamEvent::Read(sender, safe_buffer.clone(), timeout);
        self.send(event)?;
        receiver.recv()
    }

    fn write(&self, buffer: Vec<u8>) -> Result<(), Error> {
        let (sender, receiver) = channel();
        let event = StreamEvent::Write(sender, buffer);
        self.send(event)?;
        receiver.recv()
    }

    fn close_stream(self, error_code: ApplicationError, reason: Option<Vec<u8>>) {
        let event = StreamEvent::CloseStream(error_code, reason);
        self.send(event);
    }
}

impl<S : SocketConfig, T : StreamConfig> From<AsyncSender<SocketEvent<S, T>>> for Socket<S, T> {
    fn from(sender : AsyncSender<SocketEvent<S, T>>) -> Self {
        Self(sender)
    }
}
