/// Assumptions:
/// - The arguments to these functions have been checked or converted to the
///   correct format.
/// - See options.rs for compatible options and formats including certificate
///   locations.
/// -

use crate::error::{
    Error,
    ApplicationError,
};
use crate::net::Net;
use crate::options::QuicOptions;
use crate::interface::Direction;

use quinn::{
    Connecting,
    EndpointDriver,
    Endpoint,
    Incoming,
    NewConnection,
};

use futures::StreamExt;

type Result<T> = std::result::Result<T, Error>;

enum ConnectionState {
    Configured(EndpointDriver, Endpoint, Incoming),
    Connected(EndpointDriver, NewConnection),
    Error(Error),
}


impl ConnectionState {
    // On error, this should drop the connection since all the outstanding
    // references are dropped.
    fn connected(&mut self, connection : NewConnection) {
        *self = match std::mem::replace(self, Self::Error(Error::Error)) {
            Self::Configured(driver, _endpoint, _incoming) => Self::Connected(driver, connection.into()),
            _otherwise => Self::Error(Error::Error),
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

    async fn accept(&mut self) -> Option<Connecting> {
        match self {
            Self::Configured(_driver, _endpoint, incoming) => incoming.next().await,
            _ => None,
        }
    }

    fn close(&self, error_code : ApplicationError, message : &[u8]) {
        if let Self::Connected(_driver, conn) = &self {
            let NewConnection { connection: connection, .. } = conn;
            connection.close(error_code.into(), message);
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
        let sock_addr = meta.address().or()?;
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
        let sock_addr = meta.address()?;
        let conn : ConnectionState = meta
            .configure_server()
            .bind(sock_addr)?
            .into();
        Ok(Connection {
            meta: meta,
            conn: conn,
        })
    }
    
    #[cfg(try_trait)]
    async fn connect(&mut self, opts: Option<QuicOptions>) -> Result<()> {
        let server = self.meta.server_name();
        let sock_addr = self.meta.address();
        let new_conn = self.conn
            .endpoint()?
            .connect(sock_addr, server)?
            .await?;
        self.conn.connected(new_conn);
        Ok(())
    }

    // meta.new_connection moves the new_conn to a new handler.
    #[cfg(try_trait)]
    async fn accept(&mut self) -> Result<()> {
        while let new_conn = self.conn.accept()?.await? {
            self.meta.new_connection(new_conn);
        }
        Ok(())
    }

    fn open_stream(&mut self, direction : Direction) -> Result<()> {
        self.meta.new_owned_stream(direction)
    }

    fn close(&self) -> Result<()> {
        self.conn
            .close(ApplicationError::None, b"Closed");
        Ok(())
    }   
}



