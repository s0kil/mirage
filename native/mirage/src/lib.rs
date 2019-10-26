mod atoms;
mod mirage;

rustler::init! {
    "Elixir.Mirage",
    [
        mirage::from_bytes,
        mirage::resize
    ],
    load = mirage::load
}
