// use fancy_scanner;

use parser::get_basic_statement;
use parser::get_program;
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
    let mut tokens = scanner::scan_file(fp)?;

    /*
    let prog = get_program(&mut tokens);
    println!("{:#?}", prog);

    while let Some(statement) = get_basic_statement(&mut tokens) {
        println!("{:#?}", statement);
    }
    */
    Ok(())
}
