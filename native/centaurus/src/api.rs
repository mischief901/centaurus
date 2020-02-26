use rustler::Term;
use interface;

rustler_atoms! {
    atom ok;
    atom error;
    atom none;
    atom bi;
    atom uni;
}

rustler::rustler_export_nifs! {
    "Elixir.Centaurus",
    [
        ("accept_nif", 2, interface::accept),
        ("connect_nif", 2, interface::connect),
        ("close_nif", 1, interface::close),
        ("close_stream_nif", 1, interface::close_stream),
        ("listen_nif", 1, interface::listen),
        ("open_stream_nif", 2, interface::open_stream),
        ("read_nif", 2, interface::read),
        ("write_nif", 2, interface::write),
    ],
    None
}


