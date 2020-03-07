/// Creates the client and server configurations from the supplied Config data and
/// initializes the handler for the connection.

use crate::error::Error;
use crate::config::Config;
use crate::runtime::{ RunSocket, RunStream };

type Result<T> = std::result::Result<T, Error>;

/// The Connection struct ties a block of `meta` data to a connection endpoint
pub struct Connection<T : Config, H : RunSocket> {
    pub meta: T,
    pub conn: H,
}

impl <T : Config, H : RunSocket> Connection <T, H> {
    pub fn new_client(meta: T) -> Result<Self> {
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

    pub fn new_server(meta: T) -> Result<Self> {
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

    pub fn handler(&self) -> H {
        self.conn
    }
}

pub struct Stream<T, H : RunStream> {
    pub meta: T,
    pub conn: H,
}

impl <T, H : RunStream> Stream <T, H> {
    pub fn new(meta: T) -> Self {
        let conn = H::new();
        Self {
            meta,
            conn,
        }
    }

    pub fn handler(&self) -> H {
        self.conn
    }
}
