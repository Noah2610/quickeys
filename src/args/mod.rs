mod action;
mod list_args;
mod run_args;

pub use action::*;
pub use clap::Parser;
pub use list_args::*;
pub use run_args::*;

use crate::util::{expand_path_arg, Merge};
use std::path::PathBuf;

#[derive(Debug, Default, Parser)]
#[command(version, about)]
pub struct Args {
    /// Increase debug output verbosity level.
    #[arg(global = true, short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Config file location.
    /// Defaults to $XDG_CONFIG_HOME/quickeys/config.yml or platform equivalent.
    #[arg(global = true, short, long, value_name = "FILE", value_parser = expand_path_arg)]
    pub config: Option<PathBuf>,

    #[command(flatten)]
    pub run_args: RunArgs,

    #[command(subcommand)]
    pub action: Option<Action>,
}

impl Merge for Args {
    fn merge(self, other: Self) -> Self {
        Self {
            verbose: self.verbose.merge(other.verbose),
            config: self.config.merge(other.config),
            run_args: self.run_args.merge(other.run_args),
            action: self.action.merge(other.action),
        }
    }
}
