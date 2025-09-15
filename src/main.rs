mod app;
mod args;
mod config;
mod error;
mod prompt;
mod util;

use crate::util::Merge;
use app::App;
use args::{Args, Parser};
use config::Config;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error);
            ExitCode::FAILURE
        }
    }
}

fn run() -> app::Result {
    let args = Args::parse();

    let mut config = if let Some(path) = args.config.as_deref() {
        Config::from(path)
    } else {
        Config::new()
    };

    config.args.merge(args);

    let app = App::from(config);

    app.run()
}
