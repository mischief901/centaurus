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
use quinn::{ Connecting, EndpointDriver, Endpoint, Incoming, NewConnection };

use futures::stream::StreamExt;


type Result<T> = std::result::Result<T, Error>;

enum ConnectionState {
    Configured(EndpointDriver, Endpoint, Incoming),
    Connected(EndpointDriver, NewConnection),
    Error,
}

impl ConnectionState {
    // On error, this should drop the connection since all the outstanding
    // references are dropped.
    fn connected(&mut self, connection : NewConnection) {
        *self = match std::mem::replace(self, Self::Error) {
            Self::Configured(driver, _, _) => Self::Connected(driver, connection),
            _otherwise => Self::Error,
        }
    }
    
    fn endpoint(&self) -> Option<&Endpoint> {
        match self {
            Self::Configured(_driver, ref endpoint, _incoming) => Some(endpoint),
            _ => None,
        }
    }

    fn incoming(&self) -> Option<&Incoming> {
        match self {
            Self::Configured(_driver, _endpoint, ref incoming) => Some(incoming),
            _ => None,
        }
    }

    fn conn(&self) -> Option<&NewConnection> {
        match self {
            Self::Connected(_driver, ref conn) => Some(conn),
            _ => None,
        }
    }

    fn accept(&mut self) -> Option<Incoming> {
        match self {
            Self::Configured(_driver, _endpoint, incoming) => Some(&mut incoming.next()),
            _ => None,
        }
    }
}

impl From<(EndpointDriver, Endpoint, Incoming)> for ConnectionState {
    fn from((driver, endpoint, incoming) : (EndpointDriver, Endpoint, Incoming)) -> Self {
        Self::Configured(driver, endpoint, incoming)
    }
}

/// The Connection struct ties a block of `meta` data to a connection endpoint
/// and the current connection state.
pub struct Connection <T : Net> {
    meta: T,
    conn: ConnectionState,
}

impl <T : Net> Connection <T> {
    fn new_client(meta: T) -> Result<Self> {
        let sock_addr = meta.address();
        let conn : ConnectionState = meta
            .configure_client()
            .bind(sock_addr)?
            .into();
        Ok(Connection {
            meta: meta,
            conn: conn,
        })
    }

    fn new_server(meta: T) -> Result<Self> {
        let sock_addr = meta.address();
        let conn : ConnectionState = meta
            .configure_server()
            .bind(sock_addr)?
            .into();
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
    
    async fn listen(&mut self) -> Result<()> {
        while let Some(new_conn) = self.conn.accept()?.await {
            self.meta.notify(new_conn);
        }
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



