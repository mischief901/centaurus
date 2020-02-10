/// 'net.rs' provides a trait that guarantees a struct has the necessary information
/// For verifying and setting up a connection.

use std::{
    net::{
        SocketAddr,
    },
};

use quinn::{ EndpointBuilder, Connecting };

pub trait Net {
    fn address(&self) -> &SocketAddr;

    fn configure_client(&self) -> EndpointBuilder;

    fn configure_server(&self) -> EndpointBuilder;

    fn notify(&self, connection: Connecting) -> ();
    
    fn server_name(&self) -> &str;
}


