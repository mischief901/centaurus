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
    type E;
    type S;
    
    fn address(&self) -> Option<SocketAddr>;

    fn configure_client(&self) -> Result<EndpointBuilder, Self::E>;

    fn configure_server(&self) -> Result<ServerConfig, Self::E>;

    fn new_connection(&self, connection : NewConnection, ctx : &mut Context) -> ();
    
    fn server_name(&self) -> &str;

    fn new_peer_stream(&self) -> ();

    fn new_owned_stream(&self) -> Result<Self::S, Self::E>;
}


