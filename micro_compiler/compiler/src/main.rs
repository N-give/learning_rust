use scanner;
use std::fs::File;
// use std::io::prelude::*;
use std::env;

fn main() -> Result<(), std::io::Error> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("Usage: {} <FILE_NAME>", argv[0]);
        std::process::exit(-1);
    }
    let fp = File::open(argv[1].to_owned())?;
    let tokens = scanner::scan_file(fp)?;
    for t in tokens.into_iter() {
        println!("{}", t);
    }
    Ok(())
}
