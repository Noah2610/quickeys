use crate::util::{expand_path, expand_path_arg, Merge};
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, clap::Parser, Deserialize)]
pub struct RunArgs {
    /// Shell to use when running commands. Defaults to user's default shell.
    #[arg(global = true, short, long, value_name = "SHELL")]
    #[serde(default)]
    pub shell: Option<String>,

    /// Run commands in a new background process
    ///
    /// Spawns a background process using the `fork` crate's `daemon` function:
    /// https://docs.rs/fork/0.2.0/fork/fn.daemon.html
    #[arg(global = true, short, long = "bg")]
    #[serde(default)]
    pub background: bool,

    /// Optional log file for command's stdout.
    ///
    /// Creates parent directories if they do not exist.
    /// Inherits parent process' stdout if omitted.
    #[arg(global = true, long, value_parser = expand_path_arg)]
    #[serde(default, deserialize_with = "deserialize_expand_path")]
    pub stdout: Option<PathBuf>,

    /// Optional log file for command's stderr.
    ///
    /// Creates parent directories if they do not exist.
    /// Inherits parent process' stderr if omitted.
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
