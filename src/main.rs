mod app;
mod args;
mod config;
mod error;
mod prompt;
mod util;

use crate::{args::RunArgs, util::Merge};
use app::App;
use args::{Args, Parser};
use config::Config;
use serde::Deserialize;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        },
    }
}

fn run() -> app::Result {
    let Args {
        verbose,
        config: config_path,
        run_args,
        action,
    } = Args::parse();

    let mut config = if let Some(path) = config_path.as_deref() {
        Config::try_from(path)?
    } else {
        Config::local()?
    };

    config.run_args = config.run_args.merge(run_args);

    let app = App::from(config).with_verbose(verbose);

    app.run(action.unwrap_or_default())
}
