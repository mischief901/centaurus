/// The Elixir implementation of the Notify trait.
/// The notify trait is responsible for notifying the Elixir process of a new stream.

use crate::conn::{ Stream };
use crate::error::{ Error };
use crate::interface::types::{ SocketRef, StreamRef };
use crate::notify::{ Notify };

impl Notify for SocketRef {
    fn peer_bi_stream(&self, stream: Stream) -> Result<(), Error> {
        unimplemented!();
    }
    
    fn peer_uni_stream(&self, stream: Stream) -> Result<(), Error> {
        unimplemented!();
    }
}

