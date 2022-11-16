use hyper::Uri;
use std::error::Error;
use std::process::{Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Command, Child};
use tracing::{info, warn};

pub trait CameraAdapter: Send + Sync { // Extra traits so it can be shared with > 1 thread
    fn get_preview(&self) -> (Vec<u8>, String);
    fn get_camera_streams(&self) -> Vec<Uri>; //Uri does not handle RTSP URIs
}


/// Relay a pipe from a child process, by lines to a handler lambda
macro_rules! relay_pipe_lines {
    ($pipe:expr, $handler:expr) => {
        tokio::spawn(async move {
            let mut reader = BufReader::new($pipe).lines();

            loop {
                let line = reader
                    .next_line()
                    .await
                    .unwrap_or_else(|_| Some(String::new()));

                match line {
                    None => break,
                    Some(line) => $handler(line)
                }

            }
        });
    };
}

pub fn start_and_log_command(mut command: Command) -> Result<Child, Box<dyn Error>> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = command.spawn()?;

    let child_stdout = child.stdout.take().unwrap(); // remove `take` from here
    let child_stderr = child.stderr.take().unwrap(); // .. or from here and it fails
    let child_pid = child.id().unwrap();

    relay_pipe_lines!(child_stdout, |line|info!("[pid {}:stdout]: {}", child_pid, line));
    relay_pipe_lines!(child_stderr, |line|warn!("[pid {}:stderr]: {}", child_pid, line));

    Ok(child)
}

//====| Test Implementation |=======================================
use get_if_addrs::{get_if_addrs, IfAddr, Ifv4Addr};

const RTSP_SERVER: &str = "live555MediaServer";

pub struct TestCameraAdapter {
    #[allow(dead_code)]
    rtsp_server_process: Child, //TODO: listen and log to output from this, and listen for unexpected process death

    stream_uris: Vec<Uri>,
}

//TODO: error out if the child process dies
//TODO: capture and log stdout
impl TestCameraAdapter {
    pub fn new() -> Self {
        // let suffix = "sample.mkv";
        let stream_uris = get_stream_uris();

        let child = start_and_log_command(Command::new(RTSP_SERVER)).expect("Cannot start RTSP server");
        info!("Started {} with pid {}", RTSP_SERVER, child.id().unwrap());

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
                IfAddr::V4(Ifv4Addr{ip, ..}) => format!("rtsp://{}/sample.mkv", ip), //TODO: needs a test
                _ => panic!("Unexpected IP address version")
            }
        )
        .map(|str| {
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

        //FIXME: make assertions

    }
}