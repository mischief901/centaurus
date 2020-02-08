
#![feature(try_trait)]

/// This combines the various errors of quinn into a single Error enum.

use err_derive::Error;
use quinn::{
    ConnectError,
    ConnectionError,
};

#[derive(Debug, Error)]
pub enum Error {
    Error
}

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
