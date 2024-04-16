use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub profile: Profile,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub sync: ProfileSync,
    #[serde(rename = "auto-start")]
    pub autostart: bool,
    pub bin: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileSync {
    pub every: Option<String>,
}

impl Config {
    pub fn parse() -> Self {
        let path = if cfg!(debug_assertion) {
            dirs::config_dir().unwrap()
        } else {
            PathBuf::from("./tests")
        }
        .join("ramfox.toml");

        assert!(path.exists(), "Config does not exist!");

        let content = read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
