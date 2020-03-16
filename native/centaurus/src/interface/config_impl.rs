//! Contains the config impls for Quic. See centaurus::config for details.

use super::types::{
    ElixirInterface,
    ElixirStream,
    SocketAddr,
    SocketRef,
};

use crate::config::Config;
use crate::error::{ Error };

use quinn::{
    Certificate,
    CertificateChain,
    ClientConfigBuilder,
    Endpoint,
    EndpointBuilder,
    ServerConfig,
    ServerConfigBuilder,
    TransportConfig
};

use std::sync::Arc;

impl Config for SocketRef {
    type Error = Error;

    fn address(&self) -> Result<std::net::SocketAddr, Self::Error> {
        let SocketRef(lock) = self;
        match lock.read(){
            Ok(interface) => interface.address(),
            _ => Err(Error::InternalError),
        }
    }
    
    fn configure_client(&self) -> Result<EndpointBuilder, Self::Error> {
        let SocketRef(lock) = self;
        
        match lock.read(){
            Ok(interface) => interface.configure_client(),
            _ => Err(Error::InternalError),
        }
    }

    fn configure_server(&self) -> Result<EndpointBuilder, Self::Error> {
        let SocketRef(lock) = self;
        match lock.read(){
            Ok(interface) => interface.configure_server(),
            _ => Err(Error::InternalError),
        }
    }
    
    fn server_name(&self) -> Result<String, Self::Error> {
        let SocketRef(lock) = self;
        lock.read()
            .map_or(Err(Error::InternalError),
                    |interface| interface.server_name())
    }
}

impl Config for ElixirInterface {
    type Error = Error;
    
    fn address(&self) -> Result<std::net::SocketAddr, Self::Error> {
        self.socket_addr
            .map(|SocketAddr(socket)| socket)
            .to_owned()
            .ok_or(Error::InternalError)
    }

    fn configure_client(&self) -> Result<EndpointBuilder, Self::Error> {
        let mut client_builder = ClientConfigBuilder::default();
        let cert : Certificate = self.certificates
            .as_ref()
            .ok_or(Error::Error)?
            .as_cert()
            .ok_or(Error::Error)?;
        client_builder
            .add_certificate_authority(cert)
            .or(Err(Error::Error))?;
        let mut endpoint = Endpoint::builder();
        Ok(endpoint.default_client_config(client_builder.build()).to_owned())
    }

    fn configure_server(&self) -> Result<EndpointBuilder, Self::Error> {
        let server_config = ServerConfig {
            transport: Arc::new(TransportConfig {
                stream_window_uni: 0,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut server_conf_builder = ServerConfigBuilder::new(server_config);
        let cert : CertificateChain = self.certificates
            .as_ref()
            .ok_or(Error::Error)?
            .as_chain()
            .ok_or(Error::Error)?;
        let priv_key : quinn::PrivateKey = self.private_key
            .as_ref()
            .ok_or(Error::Error)?
            .as_key()
            .ok_or(Error::Error)?;
        server_conf_builder.certificate(cert, priv_key)
            .or(Err(Error::Error))?;
        let mut server_builder = Endpoint::builder();
        Ok(server_builder.listen(server_conf_builder.build()).to_owned())
    }

    fn server_name(&self) -> Result<String, Self::Error> {
        Ok(self.server_name.clone())
    }
}
