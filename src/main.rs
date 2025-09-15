mod app;
mod args;
mod config;
mod error;
mod util;

use crate::util::Merge;
use app::App;
use args::{Args, Parser};
use config::Config;

fn main() {
    match run() {
        Ok(()) => (),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
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
