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
        if let Some("der") = cert_path.extension().unwrap().to_str() {
            let cert = Certificate::from_der(&raw_certs)
                .context("Invalid Der Certificate.");
            Ok(CertificateChain::from_certs(cert))
        } else if let Some("pem") = cert_path.extension().unwrap().to_str() {
            CertificateChain::from_pem(&raw_certs)
                .context("Certificate Chain could not be derived from the provided file.")
        } else {
            Err(anyhow::anyhow!("Invalid Certificate."))
        }
    }

    pub fn as_cert(&self) -> Result<Certificate> {
        let Certificates(cert_path) = self;
        if let Some("der") = cert_path.extension().unwrap().to_str() {
            quinn::Certificate::from_der(&fs::read(cert_path).context("Certificate File not found.")?)
                .context("Invalid Der Certificate.")
        } else {
            Err(anyhow::anyhow!("Der Certificate Required."))
        }
    }
}

impl PrivateKey {
    pub fn as_key(&self) -> Result<quinn::PrivateKey> {
        let PrivateKey(path) = self;
        let raw_key = fs::read(path).ok()
            .context("Private Key File not found.")?;
        if let Some("der") = path.extension().unwrap().to_str() {
            quinn::PrivateKey::from_der(&raw_key)
                .context("Invalid Der Private Key.")
        } else if let Some("pem") = path.extension().unwrap().to_str() {
            quinn::PrivateKey::from_pem(&raw_key)
                .context("Invalid Pem Private Key.")
        } else {
            Err(anyhow::anyhow!("Invalid Private Key."))
        }
    }
}

