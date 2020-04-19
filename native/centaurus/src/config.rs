/// Provides a trait for setting up a connection.
use crate::interface::types::{ SocketRef, StreamRef };
use tokio::sync::{ RwLock };
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Configs {
    pub socket_config: Arc<RwLock<SocketRef>>,
    pub stream_config: Arc<RwLock<StreamRef>>,
}

