/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
use crate::runtime::{ Runtime };

pub mod api;
pub mod certs;
pub mod config_impl;
pub mod convert;
pub mod runtime_impl;
pub mod types;
use types::{
    ElixirInterface,
    ElixirStream,
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
    ],
    load = setup_runtime
);

fn setup_runtime(env: Env, _: Term) -> bool {
    // The Tokio runtime.
    resource!(Runtime, env);
    // The Elixir socket interface
    resource!(ElixirInterface, env);
    // The Elixir stream interface
    resource!(ElixirStream, env);
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

