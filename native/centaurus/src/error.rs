/// This combines the various errors of quinn into a single Error enum.
use quinn::{
    VarInt,
};

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
