/// Provides a trait for setting up a connection.
use crate::error::{ Error };

use std::net::SocketAddr;

use quinn::{ Certificate, CertificateChain, PrivateKey };

pub enum SocketType {
    Server,
    Client,
}

pub enum StreamType {
    Uni,
    Bi,
}

#[derive(Clone)]
pub struct Configs<S : SocketConfig, T : StreamConfig> {
    pub socket_config: S,
    pub stream_config: T,
}

pub trait SocketConfig : Send + Sync {
    // Returns the socket address to connect with.
    fn address(&self) -> Result<SocketAddr, Error>;

    // Get the certificates
    fn certs(&self) -> Result<Certificate, Error>;

    // Get the certificate chain
    fn cert_chain(&self) -> Result<CertificateChain, Error>;
    
    // Get the private key (server only)
    fn private_key(&self) -> Result<PrivateKey, Error>;

    // The server name of the connection
    fn server_name(&self) -> Result<String, Error>;
}

pub trait StreamConfig : Send + Sync {
    // 
}

