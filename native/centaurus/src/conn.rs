/// Assumptions:
/// - The arguments to these functions have been checked or converted to the
///   correct format.
/// - See options.rs for compatible options and formats including certificate
///   locations.
/// -

mod error;
mod net;
mod options;
use error::Error;
use net::Net;
use options::QuicOptions;
use quinn::{ Endpoint, Incoming, NewConnection };


type Result<T> = std::result::Result<T, Error>;

enum ConnectionState {
    Configured(Endpoint, Incoming),
    Connected(NewConnection),
    Error,
}

impl ConnectionState {
    fn configured((endpoint, incoming) : (Endpoint, Incoming)) -> Self {
        Self::Configured(endpoint, incoming)
    }

    // On error, this should drop the connection since all the outstanding
    // references are dropped.
    fn connected(&mut self, connection : NewConnection) {
        *self = match std::mem::replace(self, Self::Error) {
            Self::Configured(_, _) => Self::Connected(connection),
            _otherwise => Self::Error,
        }
    }
    
    fn endpoint(&self) -> Option<&Endpoint> {
        match self {
            Self::Configured(ref endpoint, _incoming) => Some(endpoint),
            _ => None,
        }
    }

    fn incoming(&self) -> Option<&Incoming> {
        match self {
            Self::Configured(_endpoint, ref incoming) => Some(incoming),
            _ => None,
        }
    }

    fn conn(&self) -> Option<& NewConnection> {
        match self {
            Self::Connected(ref conn) => Some(conn),
            _ => None,
        }
    }
}

/// The Connection struct ties a block of `meta` data to a connection endpoint
/// and the current connection state.
pub struct Connection <T : Net> {
    meta: T,
    conn: ConnectionState,
}

impl <T : Net> Connection <T> {
    fn new(meta: T) -> Result<Self> {
        let sock_addr = meta.address();
        let conf = meta.configure();
        let conn = conf
            .bind(sock_addr)?
            .configured();
        Ok(Connection {
            meta: meta,
            conn: conn,
        })
    }
    
    async fn connect(&mut self, opts: Option<QuicOptions>) -> Result<()> {
        let server = self.meta.server_name();
        let sock_addr = self.meta.address();
        let mut new_conn = self.conn
            .endpoint()?
            .connect(sock_addr, server)?
            .await?;
        self.conn.connected(new_conn);
        Ok(())
    }

    fn accept(&mut self) -> Result<()> {
        Ok(())
    }

    // potentially combine these and make it an option.
    fn open_bi_stream(&mut self, opts: Option<QuicOptions>) -> Result<()> {
        Ok(())
    }

    fn open_uni_stream(&mut self, opts: Option<QuicOptions>) -> Result<()> {
        Ok(())
    }

    fn close_stream(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }   
}


