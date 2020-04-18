/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::{ ApplicationError, Error };
use crate::config::{ Configs };
use crate::interface::{
    types::{ SocketType, SocketRef, StreamType, StreamRef },
};
use crate::runtime;
use crate::runtime::{ Event, SocketEvent, StreamEvent };
use crate::state::{ SocketState };

use quinn::{
    ClientConfigBuilder,
    Endpoint,
    ServerConfig,
    ServerConfigBuilder,
    TransportConfig
};

use tokio::sync::{
    Mutex,
    RwLock,
    mpsc::{
        UnboundedSender as AsyncSender,
    },
};

use std::{
    default::{ Default },
    net::{ SocketAddr },
    ops::{ Deref },
    sync::{ Arc },
    sync::mpsc::{ channel },
    time::{ Duration },
};

pub struct Socket(AsyncSender<SocketEvent>);
pub struct Stream(AsyncSender<StreamEvent>);

impl Deref for Socket {
    type Target = AsyncSender<SocketEvent>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Stream {
    type Target = AsyncSender<StreamEvent>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Socket {
    pub fn new(conn_type: SocketType, socket_config: SocketRef, stream_config: StreamRef) -> Result<Self, Error> {
        let (sender, receiver) = channel();
        let mut endpoint = Endpoint::builder();
        let socket_handle = runtime::handle()?;
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
        let configs = Configs {
            socket_config: Arc::new(RwLock::new(socket_config)),
            stream_config: Arc::new(RwLock::new(stream_config)),
        };
        let event = Event::OpenSocket(sender, conn_type, configs, state);
        socket_handle.send(event)?;
        receiver.recv()?
    }

    pub fn listen(&self, address: SocketAddr) -> Result<(), Error> {
        let (sender, receiver) = channel();
        self.send(SocketEvent::Listen(sender, address))?;
        receiver.recv()?
    }
    
    pub fn accept(&self, timeout: Option<u64>) -> Result<Self, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        self.send(SocketEvent::Accept(sender, timeout))?;
        receiver.recv()?
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

    pub fn connect(&self, address: SocketAddr, timeout: Option<u64>) -> Result<Socket, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        let event = SocketEvent::Connect(sender, address, timeout);
        self.send(event)?;
        receiver.recv()?
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
    
    pub fn new_uni_stream(&self) -> Result<Stream, Error> {
        let (sender, receiver) = channel();
        let event = SocketEvent::OpenUniStream(sender);
        self.send(event)?;
        receiver.recv()?
    }
/*      let stream = self.state
            .connection()
            .as_mut()
            .map(|connection| connection.open_uni())
            .unwrap();
        Ok(Self::Stream::new_uni_stream(self.stream_config.clone(), stream_future))
    }
*/
    pub fn new_bi_stream(&self) -> Result<Stream, Error> {
        let (sender, receiver) = channel();
        let event = SocketEvent::OpenBiStream(sender);
        self.send(event)?;
        receiver.recv()?
    }
/*        let stream = self.state
            .connection()
            .as_mut()
            .map(|connection| connection.open_bi())
            .unwrap();
        Ok(Self::Stream::new_bi_stream(self.stream_config.clone(), stream_future))
    }
*/
    pub fn close(&self, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
        let event = SocketEvent::Close(error_code, reason);
        self.send(event)?;
        Ok(())
    }
}

impl Stream {
    pub fn read(&self, buffer: Vec<u8>, timeout: Option<u64>) -> Result<u64, Error> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let safe_buffer = Arc::new(Mutex::new(buffer));
        let (sender, receiver) = channel();
        let event = StreamEvent::Read(sender, safe_buffer.clone(), timeout);
        self.send(event)?;
        receiver.recv()?
    }

    pub fn write(&self, buffer: Vec<u8>) -> Result<(), Error> {
        let (sender, receiver) = channel();
        let event = StreamEvent::Write(sender, buffer);
        self.send(event)?;
        receiver.recv()?
    }

    pub fn close_stream(&self, error_code: ApplicationError, reason: Option<Vec<u8>>) -> Result<(), Error> {
        let event = StreamEvent::CloseStream(error_code, reason);
        self.send(event)?;
        Ok(())
    }
}

impl From<AsyncSender<SocketEvent>> for Socket {
    fn from(sender : AsyncSender<SocketEvent>) -> Self {
        Self(sender)
    }
}
