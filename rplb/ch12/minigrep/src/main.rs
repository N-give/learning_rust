use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // std::env::args will panic! on any invalid unicode
    // if invalide unicode is required, use std::env::args_os

    // .collect() requires that the type be specified because it can format the
    // return value in many different ways
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
