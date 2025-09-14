mod config;

use config::Config;

fn main() {
    let config = Config::new();

    println!("{:#?}", config);
}
