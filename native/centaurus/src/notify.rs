/// This trait is a series of callback functions that are called by the Tokio runtime
/// to notify when certain actions happen.
/// Notably:
/// When a peer opens a new bidirectional/unidirectional stream.
///

use crate::error::{ Error };
use crate::conn::{ Stream };

pub trait Notify {
    /// Called when a new Bidirectional Stream is opened by the peer.
    fn new_bi_stream(&self, Stream) -> Result<(), Error>;

    /// Called when a new Unidirectional Stream is opened by the peer.
    fn new_uni_stream(&self, Stream) -> Result<(), Error>;
}

