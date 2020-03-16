/// Provides a trait for setting up a connection.
use crate::error::{ Error };

use std::net::SocketAddr;

use quinn::{ EndpointBuilder, EndpointError };

pub trait Config {
    type Error : Into<Error> + From<EndpointError>;
    // Returns the socket address to connect with.
    fn address(&self) -> Result<SocketAddr, Self::Error>;

    // Configures a client endpoint
    fn configure_client(&self) -> Result<EndpointBuilder, Self::Error>;

    // Configures a server endpoint
    fn configure_server(&self) -> Result<EndpointBuilder, Self::Error>;

    // The server name of the connection
    fn server_name(&self) -> Result<String, Self::Error>;
}

