use std::io;
use std::io::Read;

/// Generate Rust source from input XML
fn main() {
    let stdin = io::stdin();
    let envelope: media::response::Envelope = yaserde::de::from_reader(io::stdin())
        .expect("Cannot parse input");

    println!("{:#?}", envelope);
}
