/// The Elixir Interface.
/// Implements a variety of traits for setting up a Quic connection.
pub mod api;
pub mod certs;
pub mod config_impl;
pub mod convert;
pub mod types;
mod test;
use types::{
    BeamSocket,
    BeamStream,
    NewSocket,
    NewSocketInterior,
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
        test::create_cert_and_key,
        test::test_socket_config,
        test::test_stream_config,
        test::test_socket,
        test::test_stream,
        test::get_socket,
        test::get_stream,
        test::test_application_error,
        test::test_certificates,
        test::test_private_key,
        test::test_socket_addr,
        test::test_stream_type,
        test::test_socket_type,
        test::test_quic_opts,
        test::test_quic_socket,
        test::test_quic_stream,
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
    // internal new connection
    resource!(NewSocket, env);
    resource!(NewSocketInterior, env);
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

