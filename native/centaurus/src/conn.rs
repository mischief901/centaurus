/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::{ ApplicationError };
use crate::config::{ Configs };
use crate::interface::{
    types::{ SocketType, SocketRef, StreamRef },
};
use crate::runtime;
use crate::runtime::{ Event, NewSocketEvent, SocketEvent, StreamEvent };

use anyhow::{ Context, Result };

use quinn::{
    ClientConfigBuilder,
    Endpoint,
    EndpointBuilder,
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

pub struct NewSocket(pub AsyncSender<NewSocketEvent>);
pub struct Socket(pub AsyncSender<SocketEvent>);
pub struct Stream(pub AsyncSender<StreamEvent>);

impl Deref for NewSocket {
    type Target = AsyncSender<NewSocketEvent>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl NewSocket {
    pub fn new(conn_type: SocketType, socket_config: SocketRef, stream_config: StreamRef) -> Result<Self> {
        let (sender, receiver) = channel();
        let mut endpoint = Endpoint::builder();
        let socket_handle = runtime::handle()
            .context("Error getting runtime handle.")?;
        let state : EndpointBuilder = match conn_type {
            SocketType::Client => {
                let certs = socket_config.certs()?;
                let mut client = ClientConfigBuilder::default();
                client.add_certificate_authority(certs)?;
                endpoint.default_client_config(client.build());
                endpoint
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
                endpoint
            },
        };
        let configs = Configs {
            socket_config: Arc::new(RwLock::new(socket_config)),
            stream_config: Arc::new(RwLock::new(stream_config)),
        };
        let response_channel = Mutex::new(sender);
        let event = Event::OpenSocket(response_channel, conn_type, configs, state);
        socket_handle.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }
    
    pub fn accept(&self, timeout: Option<u64>) -> Result<Socket> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        self.send(NewSocketEvent::Accept(response_channel, timeout))?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }

    pub fn connect(&self, address: SocketAddr, timeout: Option<u64>) -> Result<Socket> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        let event = NewSocketEvent::Connect(response_channel, address, timeout);
        self.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }
    
    pub fn close(&self, error_code: ApplicationError, reason: Option<String>) -> Result<()> {
        let event = NewSocketEvent::Close(error_code, reason);
        self.send(event)?;
        Ok(())
    }
}

impl Socket {
    pub fn new_uni_stream(&self) -> Result<Stream> {
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        let event = SocketEvent::OpenUniStream(response_channel);
        self.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }
    
    pub fn new_bi_stream(&self) -> Result<Stream> {
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        let event = SocketEvent::OpenBiStream(response_channel);
        self.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }

    pub fn close(&self, error_code: ApplicationError, reason: Option<String>) -> Result<()> {
        let event = SocketEvent::Close(error_code, reason);
        self.send(event)?;
        Ok(())
    }
}

impl Stream {
    pub fn read(&self, buffer: Vec<u8>, timeout: Option<u64>) -> Result<usize> {
        let timeout = timeout.map(|time| Duration::from_millis(time));
        let safe_buffer = Arc::new(Mutex::new(buffer));
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        let event = StreamEvent::Read(response_channel, safe_buffer.clone(), timeout);
        self.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }

    pub fn write(&self, buffer: Vec<u8>) -> Result<()> {
        let (sender, receiver) = channel();
        let response_channel = Mutex::new(sender);
        let event = StreamEvent::Write(response_channel, buffer);
        self.send(event)?;
        receiver.recv()
            .context("Error receiving data from runtime.")?
    }

    pub fn close_stream(&self, error_code: ApplicationError) -> Result<()> {
        let event = StreamEvent::CloseStream(error_code);
        self.send(event)?;
        Ok(())
    }
}

impl From<AsyncSender<SocketEvent>> for Socket {
    fn from(sender : AsyncSender<SocketEvent>) -> Self {
        Self(sender)
    }
}

impl From<AsyncSender<NewSocketEvent>> for NewSocket {
    fn from(sender : AsyncSender<NewSocketEvent>) -> Self {
        Self(sender)
    }
}

impl From<AsyncSender<StreamEvent>> for Stream {
    fn from(sender : AsyncSender<StreamEvent>) -> Self {
        Self(sender)
    }
}
