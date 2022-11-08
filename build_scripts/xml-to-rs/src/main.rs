use std::io;

/// Generate Rust source from input XML
fn main() {

    //TODO: Allow selection of the envelope type.
    let envelope: event::response::Envelope = yaserde::de::from_reader(io::stdin())
        .expect("Cannot parse input");

    println!("{:#?}", envelope);
}
