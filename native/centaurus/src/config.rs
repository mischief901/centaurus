/// Provides a trait for setting up a connection.
use crate::error::{ Error };

use std::net::SocketAddr;

use quinn::{ Certificate, CertificateChain, PrivateKey };

#[derive(Clone)]
pub enum ConnType {
    Server,
    Client,
}

pub trait Config {
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

