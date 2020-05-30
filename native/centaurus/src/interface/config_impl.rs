//! Contains the config impls for Quic. See centaurus::config for details.

use super::types::{
    BeamSocket,
    QuicSocket,
    SocketAddr,
    SocketRef,
};

use anyhow::{ Context, Result };

use quinn::{
    Certificate,
    CertificateChain,
};

use rustler::{
    OwnedEnv,
};

impl SocketRef {
    pub fn address(&self) -> Result<std::net::SocketAddr> {
        self.0.address()
    }

    pub fn certs(&self) -> Result<Certificate> {
        self.0.certs()
    }

    pub fn cert_chain(&self) -> Result<CertificateChain> {
        self.0.cert_chain()
    }

    pub fn private_key(&self) -> Result<quinn::PrivateKey> {
        self.0.private_key()
    }

    pub fn send<T : rustler::Encoder>(&self, msg: T) -> Result<()> {
        self.0.send(msg)
    }
    
    pub fn server_name(&self) -> Result<String> {
        self.0.server_name()
    }
}

impl BeamSocket {
    fn address(&self) -> Result<std::net::SocketAddr> {
        self.bind_address
            .map(|SocketAddr(socket)| socket)
            .to_owned()
            .context("Local Socket Address is required.")
    }

    fn certs(&self) -> Result<Certificate> {
        self.certificates
            .as_ref()
            .unwrap()
            .as_cert()
            .context("Error reading Certificate")
    }

    fn cert_chain(&self) -> Result<CertificateChain> {
        self.certificates
            .as_ref()
            .unwrap()
            .as_chain()
            .context("Error reading Certificate Chain.")
    }

    fn private_key(&self) -> Result<quinn::PrivateKey> {
        self.private_key
            .as_ref()
            .unwrap()
            .as_key()
            .context("Error reading Private Key.")
    }

    fn send<T : rustler::Encoder>(&self, msg: T) -> Result<()> {
        let mut env = OwnedEnv::new();
        match &self.socket_pid {
            Some(QuicSocket::Pid(pid)) => {
                env.send_and_clear(pid, |env| msg.encode(env));
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Invalid Pid for sending data."))
        }
    }
    
    fn server_name(&self) -> Result<String> {
        Ok(self.server_name.clone())
    }
}

