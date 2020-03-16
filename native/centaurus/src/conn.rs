/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::Error;
use crate::config::Config;
use crate::runtime::{ RunSocket, RunStream };

use quinn::{ EndpointDriver, Endpoint, Incoming };

/// The Connection struct ties a block of `meta` data to a connection endpoint
pub struct Connection<T : Config, H : RunSocket + From<(EndpointDriver, Endpoint, Incoming)>> {
    pub meta: T,
    pub conn: H,
}

impl <T : Config, H : RunSocket + From<(EndpointDriver, Endpoint, Incoming)>> Connection <T, H> {
    pub fn new_client(meta: T) -> Result<Self, <T as Config>::Error> {
        let sock_addr = meta.address()?;
        let conn : H = meta
            .configure_client()?
            .bind(&sock_addr)?
            .into();
        Ok(Connection{
            meta,
            conn,
        })
    }

    pub fn new_server(meta: T) -> Result<Self, <T as Config>::Error> {
        let sock_addr = meta.address()?;
        let conn : H = meta
            .configure_server()?
            .bind(&sock_addr)?
            .into();
        Ok(Connection{
            meta,
            conn,
        })
    }

    pub fn handler(&self) -> &H {
        &self.conn
    }
}

pub struct Stream<T, H : RunStream> {
    pub meta: T,
    pub conn: H,
}

impl <T, H : RunStream> Stream <T, H> {
    pub fn new(meta: T, conn: H) -> Self {
        Self {
            meta,
            conn,
        }
    }

    pub fn handler(&self) -> &H {
        &self.conn
    }
}
