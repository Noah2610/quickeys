use crate::app::{Error, Result};
use crate::args::RunArgs;
use crate::util::expand_path_str;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Default, Debug, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default, rename = "config")]
    pub run_args: RunArgs,
    #[serde(default, deserialize_with = "deserialize_expand_path_map")]
    pub constants: HashMap<String, String>,
    #[serde(default)]
    pub keybindings: HashMap<String, String>,
}

fn deserialize_expand_path_map<'de, D>(
    d: D,
) -> std::result::Result<HashMap<String, String>, D::Error>
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
    /// Creates config from local config file if present, otherwise defaults.
    pub fn local() -> Result<Self> {
        if let Ok(file) = Self::open_config_file() {
            Self::parse_file(file)
        } else {
            Ok(Self::default())
        }
    }

    fn open_config_file() -> Result<File> {
        dirs::config_dir()
            .ok_or_else(|| {
                Error::Message("Couldn't find local config directory".into())
            })
            .and_then(|dir| {
                let path = dir.join("quickeys/config.yml");
                File::open(&path).map_err(|e| (e, path).into())
            })
    }

    fn parse_file(file: File) -> Result<Self> {
        serde_yaml::from_reader::<File, Self>(file).map_err(Into::into)
    }
}

impl TryFrom<&Path> for Config {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self> {
        File::open(path)
            .map_err(|e| (e, path).into())
            .and_then(Self::parse_file)
    }
}

// impl From<&str> for Config {
//     fn from(s: &str) -> Self {
//         serde_yaml::from_str(s).unwrap_or_default()
//     }
// }

impl TryFrom<&str> for Config {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).map_err(Into::into)
    }
}
