/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
pub mod api;
pub mod certs;
pub mod config_impl;
pub mod convert;
pub mod notify_impl;
pub mod runtime_impl;
pub mod types;
use crate::runtime;
use types::{
    BeamSocket,
    BeamStream,
    Socket,
    SocketInterior,
    Stream,
    StreamInterior,
};

use rustler::{ Env, Term };

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
    "Elixir.Centaurus.Nif",
    [
        api::get_socket_config,
        api::get_stream_config,
        api::accept,
        api::connect,
        api::close,
        api::close_stream,
        api::listen,
        api::open_stream,
        api::read,
        api::write,
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

