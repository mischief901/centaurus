/// A central struct to keep the connections current state.

use quinn::{
    Connection,
    Endpoint,
    Incoming,
    IncomingBiStreams,
    IncomingUniStreams,
    NewConnection,
    OpenBi,
    OpenUni,
    RecvStream,
    SendStream,
};

use std::{
    default::{ Default },
    sync::{ Arc, Mutex },
    ops::{ Deref },
};

#[derive(Clone)]
pub struct State(Arc<Mutex<StateInternal>>);

#[derive(Default)]
pub struct StateInternal {
    pub incoming: Option<Incoming>,
    pub endpoint: Option<Endpoint>,
    pub connection: Option<Connection>,
    pub in_uni_streams: Option<IncomingUniStreams>,
    pub in_bi_streams: Option<IncomingBiStreams>,
}

#[derive(Clone, Default)]
pub struct StreamState(Arc<Mutex<StreamStateInternal>>);

#[derive(Default)]
pub struct StreamStateInternal {
    uni_future: Option<OpenUni>,
    bi_future: Option<OpenBi>,
    pub recv: Option<RecvStream>,
    pub send: Option<SendStream>,
}

impl Deref for State {
    type Target = Mutex<StateInternal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl State {
    pub fn incoming(&self) -> Option<Incoming> {
        self.lock().unwrap().incoming.take()
    }

    pub fn endpoint(&self) -> Option<Endpoint> {
        self.lock().unwrap().endpoint.take()
    }

    pub fn connection(&self) -> Option<Connection> {
        self.lock().unwrap().connection.take()
    }

    pub fn replace<T>(&self, item: T)
    where StateInternal : Update<T> {
        self.lock().unwrap().update(item);
    }
}

/// A small trait for generically updating the state.
pub trait Update<T> {
    fn update(&mut self, item: T);
}

impl Update<Incoming> for StateInternal {
    fn update(&mut self, incoming: Incoming) {
        self.incoming.replace(incoming);
    }
}

impl Update<Endpoint> for StateInternal {
    fn update(&mut self, endpoint: Endpoint) {
        self.endpoint.replace(endpoint);
    }
}

impl Update<Connection> for StateInternal {
    fn update(&mut self, connection: Connection) {
        self.connection.replace(connection);
    }
}

impl From<NewConnection> for State {
    fn from(conn: NewConnection) -> Self {
        State(Arc::new(Mutex::new(conn.into())))
    }
}

impl From<NewConnection> for StateInternal {
    fn from(conn: NewConnection) -> Self {
        let NewConnection {
            connection,
            uni_streams,
            bi_streams,
            ..
        } = conn;
        StateInternal {
            connection: Some(connection),
            in_uni_streams: Some(uni_streams),
            in_bi_streams: Some(bi_streams),
            ..Default::default()
        }
    }
}

impl From<Endpoint> for State {
    fn from(endpoint: Endpoint) -> Self {
        State(Arc::new(Mutex::new(endpoint.into())))
    }
}

impl From<Endpoint> for StateInternal {
    fn from(endpoint: Endpoint) -> Self {
        StateInternal {
            endpoint: Some(endpoint),
            ..Default::default()
        }
    }
}

impl From<Incoming> for State {
    fn from(incoming: Incoming) -> Self {
        State(Arc::new(Mutex::new(incoming.into())))
    }
}

impl From<Incoming> for StateInternal {
    fn from(incoming: Incoming) -> Self {
        StateInternal {
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
