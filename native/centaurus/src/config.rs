/// Provides a trait for setting up a connection.
use crate::error::{ Error };

use std::net::SocketAddr;

use quinn::{ EndpointBuilder };

pub trait Config {
    type E : Into<Error>;
    // Returns the socket address to connect with.
    fn address(&self) -> Result<SocketAddr, Self::E>;

    // Configures a client endpoint
    fn configure_client(&self) -> Result<EndpointBuilder, Self::E>;

    // Configures a server endpoint
    fn configure_server(&self) -> Result<EndpointBuilder, Self::E>;

    // The server name of the connection
    fn server_name(&self) -> &str;
}

