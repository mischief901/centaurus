//! Contains the config impls for Quic. See centaurus::config for details.

use super::types::{
    ElixirInterface,
    SocketAddr,
    SocketRef,
    StreamRef,
};

use crate::conn::{ Stream };
use crate::config::{ Config };
use crate::error::{ Error };

use quinn::{
    Certificate,
    CertificateChain,
};

impl Config for SocketRef {
    fn address(&self) -> Result<std::net::SocketAddr, Error> {
        self.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.address())
    }

    fn certs(&self) -> Result<Certificate, Error> {
        self.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.certs())
    }

    fn cert_chain(&self) -> Result<CertificateChain, Error> {
        self.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.cert_chain())
    }

    fn private_key(&self) -> Result<quinn::PrivateKey, Error> {
        self.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.private_key())
    }
    
    fn server_name(&self) -> Result<String, Error> {
        self.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.server_name())
    }
}

impl Config for ElixirInterface {
    fn address(&self) -> Result<std::net::SocketAddr, Error> {
        self.socket_addr
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

