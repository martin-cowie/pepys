use regex::Regex;
use serde_derive::Deserialize;
use static_init::dynamic;
use std::fs;
use std::error::Error;

#[dynamic]
static VARIABLE_REGEX: Regex = Regex::new(r"\$\{(.+?)\}").expect("Cannot compile regex");

#[derive(Deserialize, Debug, PartialEq)]
pub enum AdapterType {
    Test,
    Pi
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub username: String,
    pub password: String,

    pub adapter_type: AdapterType,

    #[serde(rename = "pi-camera")]
    pub pi_camera: PiCameraConfig
}


#[derive(Deserialize, Debug)]
pub struct PiCameraConfig {
    command: Vec<String>,
    pub port: u16
}

impl PiCameraConfig {
    // Get the interpolated command line
    pub fn get_command(&self) -> Vec<String> {
        self.command.iter().map(|elem| {
            if let Some(captures) = VARIABLE_REGEX.captures(&elem) {
                let var_name = captures.get(1).unwrap().as_str();
                match var_name {
                    "port" => self.port.to_string(),
                    _ => panic!("Unknown variable {}", var_name)
                }
            } else {
                elem.clone()
            }
        }).collect()
    }
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        let result = toml::from_str(&content)?;
        Ok(result)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8088,
            username: "admin".to_string(),
            password: "password123".to_string(),
            adapter_type: AdapterType::Test,

            pi_camera: PiCameraConfig::default()
        }
    }
}

impl Default for PiCameraConfig {
    fn default() -> Self {
        Self {
            command: vec!["v4l2rtspserver", "-P", "${port}", "-u", "h264", "-W", "1280", "-H", "720", "/dev/video0"]
                .into_iter().map(|str| str.to_string()).collect(),
            port: 8554,
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn test_load_fails() {
        let result = Config::load("not_a_real_filename");
        assert!(matches!(result, Err(_)));
    }

    #[test]
    pub fn test_load_works() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/pepys.toml");

        let config = Config::load(d.to_str().unwrap()).unwrap();
        assert_eq!(config.port, 1234);
        assert_eq!(config.username, "some_user");
        assert_eq!(config.password, "some_password");
        assert_eq!(config.adapter_type, AdapterType::Pi);

        assert_eq!(config.pi_camera.port, 8554);
        assert_eq!(config.pi_camera.command, vec!["foo", "bar", "baz"]);
    }

    #[test]
    pub fn test_pi_camera_interp() {
        let pi_camera = PiCameraConfig {
            command: vec!["${port}".to_string(), "bar".to_string()],
            port: 1234,
        };

        let command = pi_camera.get_command();
        assert_eq!(command, vec!["1234", "bar"]);
    }

    #[test]
    #[should_panic(expected = "Unknown variable foo")]
    pub fn test_pi_camera_interp_bad() {
        let pi_camera = PiCameraConfig {
            command: vec!["${foo}".to_string(), "bar".to_string()],
            port: 1234,
        };

        pi_camera.get_command();
    }

}