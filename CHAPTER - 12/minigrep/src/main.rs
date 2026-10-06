use std::env;

use minigrep::Config;

fn main() {
    let args = env::args();

    let config = match Config::build(args) {
        Ok(config) => config,
        Err(err) => {
            println!("Problem: {err}");
            return;
        }
    };

    if let Err(e) = minigrep::run(config) {
         eprintln!("Application error: {e}");
    }
}