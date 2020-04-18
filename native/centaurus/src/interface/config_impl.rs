//! Contains the config impls for Quic. See centaurus::config for details.

use super::types::{
    BeamSocket,
    SocketAddr,
    SocketRef,
};

use crate::error::{ Error };

use quinn::{
    Certificate,
    CertificateChain,
};

impl SocketRef {
    pub fn address(&self) -> Result<std::net::SocketAddr, Error> {
        self.0.address()
    }

    pub fn certs(&self) -> Result<Certificate, Error> {
        self.0.certs()
    }

    pub fn cert_chain(&self) -> Result<CertificateChain, Error> {
        self.0.cert_chain()
    }

    pub fn private_key(&self) -> Result<quinn::PrivateKey, Error> {
        self.0.private_key()
    }
    
    pub fn server_name(&self) -> Result<String, Error> {
        self.0.server_name()
    }
}

impl BeamSocket {
    fn address(&self) -> Result<std::net::SocketAddr, Error> {
        self.bind_address
            .map(|SocketAddr(socket)| socket)
            .to_owned()
            .ok_or(Error::InternalError)
    }

    fn certs(&self) -> Result<Certificate, Error> {
        self.certificates
            .as_ref()
            .ok_or(Error::Error)?
            .as_cert()
            .ok_or(Error::Error)
    }

    fn cert_chain(&self) -> Result<CertificateChain, Error> {
        self.certificates
            .as_ref()
            .ok_or(Error::Error)?
            .as_chain()
            .ok_or(Error::Error)
    }

    fn private_key(&self) -> Result<quinn::PrivateKey, Error> {
        self.private_key
            .as_ref()
            .ok_or(Error::Error)?
            .as_key()
            .ok_or(Error::Error)
    }
    
    fn server_name(&self) -> Result<String, Error> {
        Ok(self.server_name.clone())
    }
}

