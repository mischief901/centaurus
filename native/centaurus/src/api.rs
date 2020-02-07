use rustler::Term;

rustler_atoms! {
    atom ok;
    atom error;
}

rustler::rustler_export_nifs! {
    "Elixir.Centaurus",
    [
        ("accept_nif", 2, accept),
        ("connect_nif", 4, connect),
        ("close_nif", 1, close),
        ("close_stream_nif", 1, close_stream),
        ("listen_nif", 2, listen),
        ("open_stream_nif", 2, open_stream),
        ("read_nif", 2, read),
        ("write_nif", 2, write),
    ],
    None
}

