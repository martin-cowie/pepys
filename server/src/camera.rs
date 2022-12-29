use hyper::Uri;
use regex::Regex;
use static_init::dynamic;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Command, Child};
use tracing::{info, warn, error};

use std::error::Error;
use std::process::Stdio;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;

use crate::config::PiCameraConfig;
use crate::nics;

#[dynamic]
/// A Regular expression handle IPv4 RTSP URIs with an optional port number
static RTSP_URI_REGEX: Regex = Regex::new(r"(rtsp://\d+\.\d+\.\d+\.\d+(:\d+)?)/").expect("Cannot compile regex");

const PEPYS_LOGO_BYTES: &[u8] = include_bytes!("web_services/content/pepys.jpeg");


pub trait CameraAdapter: Send + Sync { // Extra traits so it can be shared with > 1 thread
    fn get_preview(&self) -> (Vec<u8>, String);
    fn get_camera_streams(&self) -> Vec<Uri>;
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

fn monitor_child(mut child: Child) {
    tokio::spawn(async move {
        let child_pid = child.id().unwrap();
        let status = child.wait().await;
        match status {
            Ok(status) => error!("Unexpected exit of pid {}: {:?}", child_pid, status),
            Err(err) => error!("Cannot reap child: {}", err),
        }
        std::process::exit(1);
    });

}

//====| Raspberry Pi Camera |=======================================

const JPEG_MIME_TYPE: &str = "image/jpeg";

pub struct PiCameraAdapter {
    stream_uris: Vec<Uri>
}

impl PiCameraAdapter {
    pub fn new(pi_camera: &PiCameraConfig, username: &str, password: &str) -> Self {
        let args = pi_camera.get_command(username, password);
        let child = Self::start_and_log_rtsp_server(&args)
            .unwrap_or_else(|err|panic!("Cannot start RTSP server '{}': {}", args[0], err));
        let child_pid = child.id().unwrap();

        let stream_uris = Self::get_stream_uris(pi_camera.port, &pi_camera.domain);

        info!("Began RTSP server with pid {}, and RTSP URIs {:?}", child_pid, stream_uris);
        monitor_child(child);


        PiCameraAdapter {
           stream_uris,
        }
    }

    fn start_and_log_rtsp_server(args: &[String]) -> Result<Child, Box<dyn Error>> {
        let mut child = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()).spawn()?;

        let child_stdout = child.stdout.take().unwrap();
        let child_stderr = child.stderr.take().unwrap();
        let child_pid = child.id().unwrap();

        relay_pipe_lines!(child_stdout, |line: String|{
            info!("[pid {}:stdout]: {}", child_pid, line);
        });
        relay_pipe_lines!(child_stderr, |line: String|{
            warn!("[pid {}:stderr]: {}", child_pid, line);
        });

        Ok(child)
    }

    fn get_stream_uris(port_number: u16, domain: &str) -> Vec<Uri> {
        nics::get_v4_addresses()
            .map(|ip|format!("rtsp://{ip}:{port_number}/{domain}")
                .parse()
                .expect("Cannot parse RTSP URI")
            ).collect()
    }

}

impl CameraAdapter for PiCameraAdapter {
    fn get_preview(&self) -> (Vec<u8>, String) {
        let file_bytes = PEPYS_LOGO_BYTES.to_vec();
        (file_bytes, JPEG_MIME_TYPE.to_string()) //TODO: could be &str
    }

    fn get_camera_streams(&self) -> Vec<Uri> {
        self.stream_uris.clone() //TODO: replace with a reference
    }
}



//====| Test Implementation |=======================================

const RTSP_SERVER: &str = "live555MediaServer";

pub struct TestCameraAdapter {
    stream_uris: Vec<Uri>,
}

impl TestCameraAdapter {
    pub fn new() -> Self {
        let (child, mut rtsp_uri) = Self::start_and_log_test_server().expect("Cannot start RTSP server");
        let child_pid = child.id().unwrap();
        rtsp_uri.push_str("/sample.mkv");

        info!("Began {} with pid {}, and RTSP URI {}", RTSP_SERVER, child_pid, rtsp_uri);
        monitor_child(child);

        let stream_uris = vec![rtsp_uri.parse().expect("Cannot parse RTSP URI")];
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
            if !found_uri { // if let will not combin with bool shortcut
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
        let file_bytes = PEPYS_LOGO_BYTES.to_vec();
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