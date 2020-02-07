use rustler::Term;

rustler_atoms! {
    atom ok;
    atom error;
}

rustler::rustler_export_nifs! {
    "Elixir.Centaurus",
    [
        ("accept", 2, accept),
        ("connect", 4, connect),
        ("close", 1, close),
        ("close_stream", 1, close_stream),
        ("listen", 2, listen),
        ("open_stream", 2, open_stream),
        ("read", 2, read),
        ("write", 2, write),
    ],
    None
}

