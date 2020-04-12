/// This combines the various errors of quinn into a single Error enum.

use quinn::{
    ConnectError,
    ConnectionError,
    EndpointError,
    VarInt,
};

use rustler::{ NifUnitEnum };
use rustls;
use tokio::task::{ JoinError };
use tokio::time::{ Elapsed };
use webpki;

#[derive(Debug, NifUnitEnum)]
pub enum Error {
    Error,
    InternalError,
    None,
    Timeout,
}

#[cfg(try_trait)]
impl From<std::option::NoneError> for Error {
    fn from(_ : std::option::NoneError) -> Self {
        Self::Error
    }
}

impl From<rustler::Error> for Error {
    fn from(_ : rustler::Error) -> Self {
        Self::InternalError
    }
}

impl From<ConnectError> for Error {
    fn from(_ : ConnectError) -> Self {
        Self::Error
    }
}

impl From<ConnectionError> for Error {
    fn from(_ : ConnectionError) -> Self {
        Self::Error
    }
}

impl From<EndpointError> for Error {
    fn from(_ : EndpointError) -> Self {
        Self::Error
    }
}

impl From<webpki::Error> for Error {
    fn from(_ : webpki::Error) -> Self {
        Self::Error
    }
}

impl From<rustls::TLSError> for Error {
    fn from(_ : rustls::TLSError) -> Self {
        Self::Error
    }
}

impl From<Elapsed> for Error {
    fn from(_ : Elapsed) -> Self {
        Self::Timeout
    }
}

impl From<JoinError> for Error {
    fn from(_ : JoinError) -> Self {
        Self::Error
    }
}

#[derive(Debug, NifUntaggedEnum)]
pub enum ApplicationError {
    Error(u32),
    LargeError(u64),
}

impl Into<VarInt> for ApplicationError {
    fn into(self) -> VarInt {
        match self {
            ApplicationError::Error(code) => VarInt::from_u32(code),
            ApplicationError::LargeError(code) => VarInt::from_u64(code).unwrap(),
        }
    }
}
