/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
pub mod api;
pub mod certs;
pub mod config_impl;
pub mod convert;
pub mod runtime_impl;
pub mod types;
use crate::error::{ Error };
use crate::runtime;
use crate::runtime::{ Event };
use types::{
    BeamSocket,
    BeamStream,
    Socket,
    SocketInterior,
    SocketRef,
    Stream,
    StreamInterior,
    StreamRef,
};

use rustler::{ Env, Term };

use tokio::sync::mpsc::{
    unbounded_channel,
    UnboundedReceiver as AsyncReceiver,
    UnboundedSender as AsyncSender,
};

use std::sync::Once;

atoms! {
    ok,
    error,
    none,
    bi,
    uni,
    peer,
    host,
}

init!(
    "Elixir.Centaurus",
    [
        api::accept,
        api::connect,
        api::close,
        api::close_stream,
        api::listen,
        api::open_stream,
        api::read,
        api::write,
        start,
    ],
    load = setup_runtime
);

fn setup_runtime(env: Env, _: Term) -> bool {
    // The Tokio runtime.
    //    resource!(Runtime<Events<Socket, Stream>>, env);
    // The Elixir socket interface
    resource!(BeamSocket, env);
    // The Elixir stream interface
    resource!(BeamStream, env);
    // internal connection
    resource!(SocketInterior, env);
    // internal stream
    resource!(StreamInterior, env);
    // A set up quic connection
    resource!(Socket, env);
    // An open stream connection
    resource!(Stream, env);
    // Initiate the runtime.
    true
}

#[rustler::nif]
fn start() {
    handle();
}

pub fn handle() -> Result<AsyncSender<Event<SocketRef, StreamRef>>, Error> {
    static mut SENDER : Option<AsyncSender<Event<SocketRef, StreamRef>>> = None;
    static INIT : Once = Once::new();
    let sender = unsafe {
        INIT.call_once(|| {
            let (sender, receiver) : (AsyncSender<Event<SocketRef, StreamRef>>,
                                      AsyncReceiver<Event<SocketRef, StreamRef>>) = unbounded_channel();
            SENDER = Some(sender);
            std::thread::spawn(move || {
                runtime::run(receiver);
            });
        });
        SENDER.as_ref().unwrap().clone()
    };
    Ok(sender)
}
