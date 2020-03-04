/// 'net.rs' provides a trait that guarantees a struct has the necessary information
/// For verifying and setting up a connection.

use std::{
    net::{
        SocketAddr,
    },
    task::{
        Context,
    },
};

use quinn::{ EndpointBuilder, NewConnection, ServerConfig };

pub trait Net {
    // Error Type
    type E;
    // Stream Type
    type S;

    // Returns the socket address to connect with.
    fn address(&self) -> Option<SocketAddr>;

    // Configures a client endpoint
    fn configure_client(&self) -> Result<EndpointBuilder, Self::E>;

    // Configures a server endpoint
    fn configure_server(&self) -> Result<ServerConfig, Self::E>;

    // Called by the server when a new connection is accepted
    fn new_connection(&self, connection : NewConnection, ctx : &mut Context) -> ();

    // The server name of the connection
    fn server_name(&self) -> &str;

    // Called when your peer opens a new stream
    fn new_peer_stream(&self) -> Result<Self::S, Self::E>;

    // Called when you open a new stream
    fn new_owned_stream(&self) -> Result<Self::S, Self::E>;
}


