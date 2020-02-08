/// 'net.rs' provides a trait that guarantees a struct has the necessary information
/// For verifying and setting up a connection.

use std::{
    net::{
        SocketAddr,
    },
};

pub trait Net {
    // The type of quinn::ConfigBuilder to use.
    type Service;
    
    fn address(&self) -> &SocketAddr;

    fn configure(&self) -> Self::Service;

    fn server_name(&self) -> &str;
}

