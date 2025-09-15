use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

pub use clap::Parser;

#[derive(Debug, Default, Parser, Deserialize)]
#[command(version, about, long_about = None)]
#[serde(deny_unknown_fields)]
pub struct Args {
    /// Config file location.
    /// Defaults to $XDG_CONFIG_HOME/quickeys/config.yml or platform equivalent.
    #[arg(short, long, value_name = "FILE", value_parser = expand_path)]
    #[serde(skip)]
    pub config: Option<PathBuf>,

    /// Shell to use when running commands. Defaults to user's default shell.
    #[arg(short, long, value_name = "SHELL")]
    pub shell: Option<String>,

    /// Run commands in a new background process TODO
    #[arg(short, long = "bg")]
    pub background: bool,

    /// Optional log file for command's stdout.
    /// Inherits parent process' stdout by default.
    #[arg(long, value_parser = expand_path)]
    #[serde(deserialize_with = "deserialize_expand_path")]
    pub stdout: Option<PathBuf>,

    /// Optional log file for command's stderr.
    /// Inherits parent process' stderr by default.
    #[arg(long, value_parser = expand_path)]
    #[serde(deserialize_with = "deserialize_expand_path")]
    pub stderr: Option<PathBuf>,

    /// Run script for given key from config file
    #[command()]
    #[serde(skip)]
    pub key: Option<String>,
}

fn deserialize_expand_path<'de, D>(d: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(Some(
        expand_path(s.as_str())
            .map_err(|e| D::Error::custom(format!("{}", e)))?,
    ))
}

fn expand_path(
    s: &str,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Expand '~' to home path
    if s.starts_with('~') {
        if let Some(home) =
            dirs::home_dir().and_then(|p| p.to_str().map(ToString::to_string))
        {
            return Ok(PathBuf::from(s.replacen('~', home.as_str(), 1)));
        }
    };

    Ok(PathBuf::from(s))
}
