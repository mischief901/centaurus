//! Contains the impls related to Certificates and Private Keys.

use super::types::{
    Certificates,
    PrivateKey,
};

use quinn::{ Certificate, CertificateChain };

use std::fs;

impl Certificates {
    pub fn as_chain(&self) -> Option<CertificateChain> {
        let Certificates(cert_path) = self;
        let raw_certs = fs::read(cert_path).ok()?;
        if cert_path.extension().map_or(false, |x| x == "der") {
            Some(CertificateChain::from_certs(Certificate::from_der(&raw_certs)))
        } else {
            CertificateChain::from_pem(&raw_certs).ok()
        }
    }

    pub fn as_cert(&self) -> Option<Certificate> {
        let Certificates(cert_path) = self;
        quinn::Certificate::from_der(&fs::read(cert_path).ok()?)
            .ok()
    }
}

impl PrivateKey {   
    pub fn as_key(&self) -> Option<quinn::PrivateKey> {
        let PrivateKey(path) = self;
        let raw_key = fs::read(path).ok()?;
        if path.extension().map_or(false, |x| x == "der") {
            quinn::PrivateKey::from_der(&raw_key).ok()
        } else {
            quinn::PrivateKey::from_pem(&raw_key).ok()
        }
    }
}

