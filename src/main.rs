use std::{env, process};

use rgrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = rgrep::exec(config) {
        eprintln!("Application executed failed: {e}");
        process::exit(1);
    }
}


