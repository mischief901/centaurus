use crate::interface;

atoms! {
    ok,
    error,
    none,
    bi,
    uni,
}


init!(
    "Elixir.Centaurus",
    [
        interface::accept,
        interface::connect,
        interface::close,
        interface::close_stream,
        interface::listen,
        interface::open_stream,
        interface::read,
        interface::write,
    ]
);

