use hyper::Uri;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Command, Child};
use tracing::{info, warn, error};
use static_init::dynamic;

use std::error::Error;
use std::process::Stdio;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;

#[dynamic]
/// A Regular expression handle IPv4 RTSP URIs with an optional port number
static RTSP_URI_REGEX: Regex = Regex::new(r"(rtsp://\d+\.\d+\.\d+\.\d+(:\d+)?)/").expect("Cannot compile regex");

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

//====| Test Implementation |=======================================

const RTSP_SERVER: &str = "live555MediaServer";

pub struct TestCameraAdapter {
    stream_uris: Vec<Uri>,
}

impl TestCameraAdapter {
    pub fn new() -> Self {
        let (mut child, mut rtsp_uri) = Self::start_and_log_test_server().expect("Cannot start RTSP server");
        let child_pid = child.id().unwrap();
        rtsp_uri.push_str("/sample.mkv");

        info!("Started {} with pid {}, and RTSP end point {}", RTSP_SERVER, child_pid, rtsp_uri);

        let stream_uris = vec![rtsp_uri.parse().expect("Cannot parse RTSP URI")];

        tokio::spawn(async move {
            let status = child.wait().await;
            match status {
                Ok(status) => error!("Unexpected exit of {} pid {}: {:?}", RTSP_SERVER, child_pid, status),
                Err(err) => error!("Cannot reap child: {}", err),
            }
            std::process::exit(1);
        });

        Self {
            stream_uris
        }
    }

    /// Start the test server, and return the child process and the URI at which it listens
    fn start_and_log_test_server() -> Result<(Child, String), Box<dyn Error>> {
        let mut child = Command::new(RTSP_SERVER)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()).spawn()?;

        let child_stdout = child.stdout.take().unwrap();
        let child_stderr = child.stderr.take().unwrap();
        let child_pid = child.id().unwrap();
        let mut found_uri = false;

        relay_pipe_lines!(child_stdout, |line: String|{
            info!("[pid {}:stdout]: {}", child_pid, line);
        });

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        relay_pipe_lines!(child_stderr, |line: String |{
            if found_uri == false { // if let will not combin with bool shortcut
                if let Some(captures) = RTSP_URI_REGEX.captures(&line) {
                    let rtsp_uri = captures.get(1).unwrap().as_str().to_string();
                    tx.send(rtsp_uri).expect("Cannot send URI via channel");
                    found_uri = true
                }
            }

            warn!("[pid {}:stderr]: {}", child_pid, line);
        });

        // Gather the URI from the stderr listener/logger
        let rtsp_uri = rx.recv_timeout(Duration::from_secs(5)).expect("Cannot receive URI via channel");

        Ok((child, rtsp_uri))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex() {
        let result = RTSP_URI_REGEX.captures(r#"rtsp://192.168.59.27/<filename>"#);
        assert!(matches!(result, Some(caps) if caps.get(1).unwrap().as_str() == "rtsp://192.168.59.27"));

        let result = RTSP_URI_REGEX.captures(r#"rtsp://192.168.59.27:8554/<filename>"#);
        assert!(matches!(result, Some(caps) if caps.get(1).unwrap().as_str() == "rtsp://192.168.59.27:8554"));
    }
}