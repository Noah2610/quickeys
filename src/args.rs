use std::path::PathBuf;

pub use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Config file location.
    /// Defaults to $XDG_CONFIG_HOME/quickeys/config.yml or platform equivalent.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Run script for given key from config file
    #[command()]
    pub key: Option<String>,
}
