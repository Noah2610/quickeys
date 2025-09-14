mod app;
mod args;
mod config;
mod error;

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

    let config = if let Some(path) = args.config.as_deref() {
        Config::from(path)
    } else {
        Config::new()
    };

    let app = App::from(config);

    if let Some(key) = args.key.as_deref() {
        app.run_key(key)
    } else {
        app.run()
    }
}
