use crate::args::Args;
use crate::util::expand_path_str;
use serde::{de::Error, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Default, Debug, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "config")]
    pub args: Args,
    #[serde(deserialize_with = "deserialize_expand_path_map")]
    pub constants: HashMap<String, String>,
    pub keybindings: HashMap<String, String>,
}

fn deserialize_expand_path_map<'de, D>(
    d: D,
) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut map = HashMap::<String, String>::deserialize(d)?;

    for value in map.values_mut() {
        *value = expand_path_str(value.as_str());
    }

    Ok(map)
}

impl Config {
    /// Creates config from local config files, if present, or defaults.
    pub fn new() -> Self {
        Self::open_config_file()
            .and_then(Self::parse_file)
            .unwrap_or_default()
    }

    fn open_config_file() -> Option<File> {
        dirs::config_dir()
            .and_then(|dir| File::open(dir.join("quickeys/config.yml")).ok())
    }

    fn parse_file(file: File) -> Option<Self> {
        match serde_yaml::from_reader::<File, Self>(file) {
            Ok(config) => Some(config),
            Err(err) => {
                eprintln!("Failed to deserialize config file:\n{:}", err);
                None
            }
        }
    }
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        File::open(path)
            .ok()
            .and_then(Self::parse_file)
            .unwrap_or_default()
    }
}
