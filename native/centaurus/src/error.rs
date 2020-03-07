/// This combines the various errors of quinn into a single Error enum.

use quinn::{
    ConnectError,
    ConnectionError,
    EndpointError,
    VarInt,
};

use rustler_codegen::NifUnitEnum;

#[derive(Debug, NifUnitEnum)]
pub enum Error {
    Error,
    InternalError,
    None,
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
