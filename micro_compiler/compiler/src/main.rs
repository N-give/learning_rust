// use fancy_scanner;

use parser::parse_file;
use scanner;
use std::env;
use std::fs::File;
// use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("Usage: {} <FILE_NAME>", argv[0]);
        std::process::exit(-1);
    }
    let fp = File::open(argv[1].to_owned())?;
    let tokens = scanner::scan_file(fp)?;
    parse_file(tokens);
    Ok(())
}
