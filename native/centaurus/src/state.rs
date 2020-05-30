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
    pub local: Option<Arc<Mutex<SocketStateLocal>>>,
    pub peer: SocketStatePeer,
}

#[derive(Default)]
pub struct SocketStateLocal {
    pub incoming: Option<Incoming>,
    pub endpoint: Option<Endpoint>,
    pub connection: Option<Connection>,
}

#[derive(Clone, Default)]
pub struct SocketStatePeer {
    pub uni_streams: Option<Arc<Mutex<IncomingUniStreams>>>,
    pub bi_streams: Option<Arc<Mutex<IncomingBiStreams>>>,
}

#[derive(Clone, Debug, Default)]
pub struct StreamState {
    pub local: Option<Arc<Mutex<StreamStateLocal>>>,
//    pub peer: Option<Arc<StreamStatePeer>>,
}

#[derive(Default)]
pub struct StreamStateLocal {
    pub bi_future: Option<OpenBi>,
    pub uni_future: Option<OpenUni>,
    pub recv: Option<RecvStream>,
    pub send: Option<SendStream>,
}

// Currently not used, but will be used when active receive is implemented.
#[derive(Default)]
pub struct StreamStatePeer;

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

impl fmt::Debug for StreamStateLocal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Local Stream State Error.")
    }
}

impl fmt::Debug for StreamStatePeer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peer Stream State Error.")
    }
}

impl From<(Endpoint, Incoming)> for SocketState {
    fn from(local: (Endpoint, Incoming)) -> Self {
        let local = Some(Arc::new(Mutex::new(local.into())));
        Self {
            local,
            ..Default::default()
        }
    }
}

impl From<(Endpoint, Incoming)> for SocketStateLocal {
    fn from((endpoint, incoming) : (Endpoint, Incoming)) -> Self {
        Self {
            endpoint: Some(endpoint),
            incoming: Some(incoming),
            ..Default::default()
        }
    }
}

impl From<NewConnection> for SocketState {
    fn from(conn: NewConnection) -> Self {
        let local = Some(Arc::new(Mutex::new(
            SocketStateLocal {
                connection: Some(conn.connection),
                ..Default::default()
            })));
        let peer = SocketStatePeer {
            uni_streams: Some(Arc::new(Mutex::new(conn.uni_streams))),
            bi_streams: Some(Arc::new(Mutex::new(conn.bi_streams))),
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
            local: Some(Arc::new(Mutex::new(endpoint.into()))),
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
            local: Some(Arc::new(Mutex::new(incoming.into()))),
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
        StreamState {
            local: Some(Arc::new(Mutex::new(future.into()))),
            ..Default::default()
        }
    }
}

impl From<OpenUni> for StreamStateLocal {
    fn from(future: OpenUni) -> Self {
        StreamStateLocal {
            uni_future: Some(future),
            ..Default::default()
        }
    }
}
        
impl From<OpenBi> for StreamState {
    fn from(future: OpenBi) -> Self {
        StreamState {
            local: Some(Arc::new(Mutex::new(future.into()))),
            ..Default::default()
        }
    }
}

impl From<OpenBi> for StreamStateLocal {
    fn from(future: OpenBi) -> Self {
        StreamStateLocal {
            bi_future: Some(future),
            ..Default::default()
        }
    }
}

impl From<SendStream> for StreamState {
    fn from(send: SendStream) -> Self {
        let local = send.into();
        StreamState {
            local: Some(Arc::new(Mutex::new(local))),
            ..Default::default()
        }
    }
}

impl From<SendStream> for StreamStateLocal {
    fn from(send: SendStream) -> Self {
        StreamStateLocal {
            send: Some(send),
            ..Default::default()
        }
    }
}

impl From<RecvStream> for StreamState {
    fn from(recv: RecvStream) -> Self {
        let local = recv.into();
        StreamState {
            local: Some(Arc::new(Mutex::new(local))),
            ..Default::default()
        }
    }
}

impl From<RecvStream> for StreamStateLocal {
    fn from(recv: RecvStream) -> Self {
        StreamStateLocal {
            recv: Some(recv),
            ..Default::default()
        }
    }
}

impl From<(SendStream, RecvStream)> for StreamState {
    fn from(bi_stream : (SendStream, RecvStream)) -> Self {
        let local = bi_stream.into();
        Self {
            local: Some(Arc::new(Mutex::new(local))),
            ..Default::default()
        }
    }
}

impl From<(SendStream, RecvStream)> for StreamStateLocal {
    fn from((send, recv) : (SendStream, RecvStream)) -> Self {
        Self {
            send: Some(send),
            recv: Some(recv),
            ..Default::default()
        }
    }
}
