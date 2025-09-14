use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub constants: HashMap<String, String>,
    pub keybindings: HashMap<String, String>,
}

impl Config {
    /// Creates config from local config files, if present, or defaults.
    pub fn new() -> Self {
        if let Some(file) = Self::open_config_file() {
            match serde_yaml::from_reader::<File, Self>(file) {
                Ok(config) => return config,
                Err(err) => {
                    eprintln!("Failed to deserialize config file:\n{:#?}", err)
                }
            }
        }

        Self::default()
    }

    fn open_config_file() -> Option<File> {
        dirs::config_dir()
            .and_then(|dir| File::open(dir.join("quickeys/config.yml")).ok())
    }
}
