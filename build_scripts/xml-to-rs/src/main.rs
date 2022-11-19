use std::io;
use std::error::Error;

/// Generate Rust source from input XML
fn main() -> Result<(), Box<dyn Error>> {

    let mut args = std::env::args();
    let argv0 = args.next().expect("Cannot get argv[0]");

    let package_name = args
        .next()
        .unwrap_or_else(||panic!("wrong # args, try {argv0} (event|devicemgmt|media) < some-xml"));

    match package_name.as_ref() {
        "event" => {
            let envelope: event::response::Envelope = yaserde::de::from_reader(io::stdin())?;
            println!("{:#?}", envelope);
        }
        "devicemgmt" => {
            let envelope: devicemgmt::response::Envelope = yaserde::de::from_reader(io::stdin())?;
            println!("{:#?}", envelope);
        }
        "media" => {
            let envelope: media::response::Envelope = yaserde::de::from_reader(io::stdin())?;
            println!("{:#?}", envelope);
        }

        _ => {
            panic!("Unexpected envelope type '{package_name}', try {argv0} (event|devicemgmt|media) < some-xml");
        }
    };

    Ok(())

}
