/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
use crate::conn;
use crate::runtime::{ Handle, RunSocket, RunStream, Runtime };

pub mod api;
pub mod certs;
pub mod config_impl;
pub mod convert;
// TODO: mod options;
pub mod runtime_impl;
pub mod types;
use types::{
    Connection,
    ElixirInterface,
    ElixirStream,
    SocketRef,
    SocketHandler,
    Stream,
    StreamHandler,
    StreamRef,
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
    // Handles to jobs in the Tokio runtime.
    resource!(Handle, env);
    // The Elixir socket interface
    resource!(ElixirInterface, env);
    // The Elixir stream interface
    resource!(ElixirStream, env);
    // internal connection
    resource!(conn::Connection<SocketRef, SocketHandler>, env);
    // internal stream
    resource!(conn::Stream<StreamRef, StreamHandler>, env);
    // A set up quic connection
    resource!(Connection, env);
    // An open stream connection
    resource!(Stream, env);

    // Initiate the runtime.
    Runtime::new();
    true
}

