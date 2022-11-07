use tracing::info;
use hyper::Uri;

pub trait CameraAdapter: Send + Sync { // Extra traits so it can be shared with > 1 thread
    fn get_preview(&self) -> (Vec<u8>, String);
    fn get_camera_streams(&self) -> Vec<Uri>; //Uri does not handle RTSP URIs
}

//====| Test Implementation |=======================================
use std::process::{Command, Child};
use get_if_addrs::{get_if_addrs, IfAddr, Ifv4Addr};

const TEST_STREAM_URI: &str = "rtsp://192.168.59.27/sample.mkv";
const RTSP_SERVER: &str = "live555MediaServer";

pub struct TestCameraAdapter {
    #[allow(dead_code)]
    rtsp_server_process: Child, //TODO: listen and log to output from this, and listen for unexpected process death

    stream_uris: Vec<Uri>,
}

//TODO: error out if the child process dies
//TODO: capture and log stdout
//TODO: replace hardcoded IP address
impl TestCameraAdapter {
    pub fn new() -> Self {
        info!("Build a TestCameraAdapter exposing RTSP stream at {}", TEST_STREAM_URI);

        // let suffix = "sample.mkv";
        let stream_uris = get_stream_uris();

        let child = Command::new(RTSP_SERVER).spawn().expect("Cannot start RTSP server");
        info!("Started {} with pid {}", RTSP_SERVER, child.id());

        Self {
            rtsp_server_process: child,
            stream_uris
        }
    }
}

impl CameraAdapter for TestCameraAdapter {

    /// Returns the preview image and its MIME type
    fn get_preview(&self) -> (Vec<u8>, String) {
        let file_bytes = include_bytes!("preview.jpeg").to_vec();
        (file_bytes, "image/jpeg".to_string())
    }

    fn get_camera_streams(&self) -> Vec<Uri> {
        self.stream_uris.clone() //TODO: replace with a reference
    }
}

fn get_stream_uris() -> Vec<Uri> {
    let result: Vec<Uri> = get_if_addrs().expect("Cannot get NICs")
        .into_iter()
        .filter(|nic| !nic.is_loopback() && !matches!(nic.addr, IfAddr::V6(_)) )
        .map(|nic|
            match nic.addr {
                IfAddr::V4(Ifv4Addr{ip, ..}) => format!("rtsp://{}/sample.mkv", ip),
                _ => panic!("Unexpected IP address version")
            }
        )
        .map(|str| {
            dbg!(&str);
            str.parse().expect("Cannot parse RTSP URI")
        })
        .collect();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uri_parse() {
        let uri: Uri = "rtsp://foo/bar".parse().expect("Cannot parse URI");

        dbg!(uri);

        let uris = get_stream_uris();

        dbg!(uris);

    }
}