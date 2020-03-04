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

pub trait Net<E> {
    
    fn address(&self) -> Option<SocketAddr>;

    fn configure_client(&self) -> Result<EndpointBuilder, E>;

    fn configure_server(&self) -> Result<ServerConfig, E>;

    fn notify(&self, connection : NewConnection, ctx : &mut Context) -> ();
    
    fn server_name(&self) -> &str;
}


