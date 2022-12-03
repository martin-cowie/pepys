use serde_derive::Deserialize;
use std::fs;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub username: String,
    pub password: String,
    pub rtsp_server_command: String
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
            rtsp_server_command: "live555MediaServer".to_string()
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
    }

}