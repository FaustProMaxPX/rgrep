use std::{env, process};

use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = rgrep::exec(config) {
        println!("Application executed failed: {e}");
        process::exit(1);
    }
}


