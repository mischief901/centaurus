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
        test::encode_socket_config,
        test::encode_stream_config,
        test::decode_socket_config,
        test::decode_stream_config,
        test::encode_socket,
        test::encode_stream,
        test::decode_socket,
        test::decode_stream,
        test::encode_application_error,
        test::decode_application_error,
        test::encode_certificates,
        test::decode_certificates,
        test::encode_private_key,
        test::decode_private_key,
        test::encode_socket_addr,
        test::decode_socket_addr,
        test::encode_stream_type,
        test::decode_stream_type,
        test::encode_socket_type,
        test::decode_socket_type,
        test::encode_quic_opts,
        test::decode_quic_opts,
        test::encode_quic_socket,
        test::encode_quic_stream,
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

