use std::io;

/// Generate Rust source from input XML
fn main() {
    let envelope: media::response::Envelope = yaserde::de::from_reader(io::stdin())
        .expect("Cannot parse input");

    println!("{:#?}", envelope);
}
