//! Contains the impls related to Certificates and Private Keys.

use super::types::{
    Certificates,
    PrivateKey,
};

use anyhow::{ Context, Result };

use quinn::{ Certificate, CertificateChain };

use std::fs;

impl Certificates {
    pub fn as_chain(&self) -> Result<CertificateChain> {
        let Certificates(cert_path) = self;
        let raw_certs = fs::read(cert_path)?;
        if cert_path.extension().map_or(false, |x| x == "der") {
            let cert = Certificate::from_der(&raw_certs)
                .context("Invalid Der Certificate.");
            Ok(CertificateChain::from_certs(cert))
        } else {
            CertificateChain::from_pem(&raw_certs)
                .context("Certificate Chain could not be derived from the provided file.")
        }
    }

    pub fn as_cert(&self) -> Result<Certificate> {
        let Certificates(cert_path) = self;
        quinn::Certificate::from_der(&fs::read(cert_path).context("Certificate File not found.")?)
            .context("Invalid Der Certificate.")
    }
}

impl PrivateKey {
    pub fn as_key(&self) -> Result<quinn::PrivateKey> {
        let PrivateKey(path) = self;
        let raw_key = fs::read(path).ok()
            .context("Private Key File not found.")?;
        if path.extension().map_or(false, |x| x == "der") {
            quinn::PrivateKey::from_der(&raw_key)
                .context("Invalid Der Private Key.")
        } else {
            quinn::PrivateKey::from_pem(&raw_key)
                .context("Invalid Pem Private Key.")
        }
    }
}

