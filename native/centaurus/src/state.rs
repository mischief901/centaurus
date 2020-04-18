/// A central struct to keep the connections current state.

use quinn::{
    Connection,
    Endpoint,
    EndpointBuilder,
    Incoming,
    IncomingBiStreams,
    IncomingUniStreams,
    NewConnection,
    OpenBi,
    OpenUni,
    RecvStream,
    SendStream,
};

use tokio::sync::{ Mutex };

use std::{
    default::{ Default },
    sync::{ Arc },
    ops::{ Deref },
};

#[derive(Clone)]
pub struct SocketState(Arc<Mutex<SocketStateInternal>>);

#[derive(Default)]
pub struct SocketStateInternal {
    pub incoming: Option<Incoming>,
    pub endpoint: Option<Endpoint>,
    pub endpoint_builder: Option<EndpointBuilder>,
    pub connection: Option<Connection>,
    pub in_uni_streams: Option<IncomingUniStreams>,
    pub in_bi_streams: Option<IncomingBiStreams>,
}

#[derive(Clone)]
pub struct StreamState(Arc<Mutex<StreamStateInternal>>);

#[derive(Default)]
pub struct StreamStateInternal {
    bi_future: Option<OpenBi>,
    uni_future: Option<OpenUni>,
    pub recv: Option<RecvStream>,
    pub send: Option<SendStream>,
}

impl Deref for SocketState {
    type Target = Mutex<SocketStateInternal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EndpointBuilder> for SocketState {
    fn from(endpoint: EndpointBuilder) -> Self {
        SocketState(Arc::new(Mutex::new(endpoint.into())))
    }
}

impl From<EndpointBuilder> for SocketStateInternal {
    fn from(endpoint_builder: EndpointBuilder) -> Self {
        SocketStateInternal {
            endpoint_builder: Some(endpoint_builder),
            ..Default::default()
        }
    }
}

impl From<NewConnection> for SocketState {
    fn from(conn: NewConnection) -> Self {
        SocketState(Arc::new(Mutex::new(conn.into())))
    }
}

impl From<NewConnection> for SocketStateInternal {
    fn from(conn: NewConnection) -> Self {
        let NewConnection {
            connection,
            uni_streams,
            bi_streams,
            ..
        } = conn;
        SocketStateInternal {
            connection: Some(connection),
            in_uni_streams: Some(uni_streams),
            in_bi_streams: Some(bi_streams),
            ..Default::default()
        }
    }
}

impl From<Endpoint> for SocketState {
    fn from(endpoint: Endpoint) -> Self {
        SocketState(Arc::new(Mutex::new(endpoint.into())))
    }
}

impl From<Endpoint> for SocketStateInternal {
    fn from(endpoint: Endpoint) -> Self {
        SocketStateInternal {
            endpoint: Some(endpoint),
            ..Default::default()
        }
    }
}

impl From<Incoming> for SocketState {
    fn from(incoming: Incoming) -> Self {
        SocketState(Arc::new(Mutex::new(incoming.into())))
    }
}

impl From<Incoming> for SocketStateInternal {
    fn from(incoming: Incoming) -> Self {
        SocketStateInternal {
            incoming: Some(incoming),
            ..Default::default()
        }
    }
}

impl From<OpenUni> for StreamState {
    fn from(future: OpenUni) -> Self {
        StreamState(Arc::new(Mutex::new(future.into())))
    }
}

impl From<OpenUni> for StreamStateInternal {
    fn from(future: OpenUni) -> Self {
        StreamStateInternal {
            uni_future: Some(future),
            ..Default::default()
        }
    }
}
        
impl From<OpenBi> for StreamState {
    fn from(future: OpenBi) -> Self {
        StreamState(Arc::new(Mutex::new(future.into())))
    }
}

impl From<OpenBi> for StreamStateInternal {
    fn from(future: OpenBi) -> Self {
        StreamStateInternal {
            bi_future: Some(future),
            ..Default::default()
        }
    }
}
