use regex::{Regex, Captures};

use serde_derive::Deserialize;
use static_init::dynamic;
use std::fs;
use std::error::Error;

#[dynamic]
static VARIABLE_REGEX: Regex = Regex::new(r"\$\{(\w+)\}").expect("Cannot compile regex");

#[derive(Deserialize, Debug, PartialEq)]
pub enum AdapterType {
    Test,
    Pi
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Config {
    pub port: u16,
    pub username: String,
    pub password: String,

    pub adapter_type: AdapterType,

    #[serde(rename = "pi-camera")]
    pub pi_camera: PiCameraConfig
}


#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct PiCameraConfig {
    pub command: Vec<String>,
    pub domain: String,
    pub port: u16
}

impl PiCameraConfig {
    // Get the interpolated command line
    pub fn get_command(&self, username: &str, password: &str) -> Vec<String> {
        self.command.iter().map(|elem| {
            let result = VARIABLE_REGEX.replace_all(elem, |caps: &Captures| {
                let var_name = &caps[1];
                match var_name {
                    "domain" => self.domain.to_string(),
                    "password" => password.to_string(),
                    "port" => self.port.to_string(),
                    "user" => username.to_string(),
                    _ => panic!("Unexpected variable {var_name}")
                }
            });
            result.to_string()
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
            command: vec!["v4l2rtspserver", "-P", "${port}", "-u", "${domain}", "-W", "1280", "-H", "720", "-U", "${user}:${password}", "/dev/video0"]
                .into_iter().map(|str| str.to_string()).collect(),
            domain: "camera".into(),
            port: 8554,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use regex::Captures;

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
    pub fn test_defaults() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/empty.toml");

        let config = Config::load(d.to_str().unwrap()).unwrap();
        let defaults = Config::default();

        assert_eq!(config, defaults);
    }

    #[test]
    pub fn test_pi_camera_interp() {
        let pi_camera = PiCameraConfig {
            command: vec!["[${port}]".to_string(), "bar".to_string(), "${password}-${user}".to_string()],
            domain: "some_domain".into(),
            port: 1234,
        };

        let command = pi_camera.get_command("some_user", "some_password");
        assert_eq!(command, vec!["[1234]", "bar", "some_password-some_user"]);
    }

    #[test]
    #[should_panic(expected = "Unexpected variable foo")]
    pub fn test_pi_camera_interp_bad() {
        let pi_camera = PiCameraConfig {
            command: vec!["${foo}".to_string(), "bar".to_string()],
            domain: "some_domain".into(),
            port: 1234,
        };

        pi_camera.get_command("user", "passwd");
    }

    #[test]
    pub fn test_regex_replace() {
        let result = VARIABLE_REGEX.replace("foo ${bar} baz", |caps: &Captures| {
            format!("[{}]", &caps[1])
        });
        assert_eq!(result, "foo [bar] baz");

        let result = VARIABLE_REGEX.replace("foo bar baz", |caps: &Captures| {
            format!("[{}]", &caps[1])
        });
        assert_eq!(result, "foo bar baz");
    }

}