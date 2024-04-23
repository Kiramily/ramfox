use std::fs::read_to_string;

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
        let path = dirs::config_dir().unwrap().join("ramfox.toml");

        assert!(path.exists(), "Config does not exist!");

        let content = read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
