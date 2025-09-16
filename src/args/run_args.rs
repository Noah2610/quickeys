use crate::util::{expand_path, expand_path_arg, Merge};
use clap::Parser;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

#[derive(Debug, Default, Parser, Deserialize)]
#[command()]
pub struct RunArgs {
    /// Shell to use when running commands. Defaults to user's default shell.
    #[arg(global = true, short, long, value_name = "SHELL")]
    #[serde(default)]
    pub shell: Option<String>,

    /// Run commands in a new background process TODO
    #[arg(global = true, short, long = "bg")]
    #[serde(default)]
    pub background: bool,

    /// Optional log file for command's stdout.
    ///
    /// Inherits parent process' stdout by default.
    #[arg(global = true, long, value_parser = expand_path_arg)]
    #[serde(default, deserialize_with = "deserialize_expand_path")]
    pub stdout: Option<PathBuf>,

    /// Optional log file for command's stderr.
    ///
    /// Inherits parent process' stderr by default.
    #[arg(global = true, long, value_parser = expand_path_arg)]
    #[serde(default, deserialize_with = "deserialize_expand_path")]
    pub stderr: Option<PathBuf>,
}

impl Merge for RunArgs {
    fn merge(self, other: Self) -> Self {
        Self {
            shell: self.shell.merge(other.shell),
            background: self.background.merge(other.background),
            stdout: self.stdout.merge(other.stdout),
            stderr: self.stderr.merge(other.stderr),
        }
    }
}

fn deserialize_expand_path<'de, D>(d: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(Some(expand_path(s.as_str())))
}
