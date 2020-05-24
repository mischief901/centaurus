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
    fmt,
    sync::{ Arc },
    ops::{ Deref },
};

#[derive(Clone, Debug, Default)]
pub struct SocketState {
    local: Arc<Mutex<Option<SocketStateLocal>>>,
    pub peer: SocketStatePeer,
}

#[derive(Default)]
pub struct SocketStateLocal {
    pub incoming: Option<Incoming>,
    pub endpoint: Option<Endpoint>,
    pub endpoint_builder: Option<EndpointBuilder>,
    pub connection: Option<Connection>,
}

#[derive(Clone, Default)]
pub struct SocketStatePeer {
    pub uni_streams: Arc<Mutex<Option<IncomingUniStreams>>>,
    pub bi_streams: Arc<Mutex<Option<IncomingBiStreams>>>,
}

#[derive(Clone, Debug, Default)]
pub struct StreamState(Arc<Mutex<StreamStateInternal>>);

#[derive(Default)]
pub struct StreamStateInternal {
    bi_future: Option<OpenBi>,
    uni_future: Option<OpenUni>,
    pub recv: Option<RecvStream>,
    pub send: Option<SendStream>,
}

impl fmt::Debug for SocketStateLocal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Local Socket State Error.")
    }
}

impl fmt::Debug for SocketStatePeer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peer Socket State Error.")
    }
}

impl fmt::Debug for StreamStateInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal Stream State Error.")
    }
}

impl From<EndpointBuilder> for SocketState {
    fn from(endpoint: EndpointBuilder) -> Self {
        SocketState {
            local: Arc::new(Mutex::new(Some(endpoint.into()))),
            ..Default::default()
        }
    }
}

impl From<EndpointBuilder> for SocketStateLocal {
    fn from(endpoint_builder: EndpointBuilder) -> Self {
        SocketStateLocal {
            endpoint_builder: Some(endpoint_builder),
            ..Default::default()
        }
    }
}

impl From<NewConnection> for SocketState {
    fn from(conn: NewConnection) -> Self {
        let local = Arc::new(Mutex::new(Some(
            SocketStateLocal {
                connection: Some(conn.connection),
                ..Default::default()
            })));
        let peer = SocketStatePeer {
            uni_streams: Arc::new(Mutex::new(Some(conn.uni_streams))),
            bi_streams: Arc::new(Mutex::new(Some(conn.bi_streams))),
            ..Default::default()
        };
        SocketState {
            local,
            peer,
        }
    }
}

impl From<Endpoint> for SocketState {
    fn from(endpoint: Endpoint) -> Self {
        SocketState {
            local: Arc::new(Mutex::new(Some(endpoint.into()))),
            ..Default::default()
        }
    }
}

impl From<Endpoint> for SocketStateLocal {
    fn from(endpoint: Endpoint) -> Self {
        SocketStateLocal {
            endpoint: Some(endpoint),
            ..Default::default()
        }
    }
}

impl From<Incoming> for SocketState {
    fn from(incoming: Incoming) -> Self {
        SocketState {
            local: Arc::new(Mutex::new(Some(incoming.into()))),
            ..Default::default()
        }
    }
}

impl From<Incoming> for SocketStateLocal {
    fn from(incoming: Incoming) -> Self {
        SocketStateLocal {
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
