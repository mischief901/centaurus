/// This combines the various errors of quinn into a single Error enum.

use quinn::{
    ConnectError,
    ConnectionError,
    EndpointError,
    VarInt,
};

use quinn_proto::Code as ErrorCode;

#[derive(Debug)]
pub enum Error {
    Error,
    None,
}

#[cfg(try_trait)]
impl From<std::option::NoneError> for Error {
    fn from(_ : std::option::NoneError) -> Self {
        Self::Error
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

pub enum ApplicationError {
    Quic(QuicError),
    None
}

impl Into<VarInt> for ApplicationError {
    fn into(self) -> VarInt {
        match self {
            Self::Quic(error_code) => error_code,
            Self::None => 
        }
    }
}
