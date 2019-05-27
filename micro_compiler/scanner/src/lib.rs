extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;

// kjkj
static REGEX_IDENTIFIER: &str = r"\w[a-zA-Z0-9]+";
// static REGEX_INTLITERAl: &str = r"\d+";
// static REGEX_FLOATLITER: &str = r"\d+\.\d+";
// static REGEX_STRINGLITERAL: &str = r"";

pub fn scan_file(fp: std::fs::File) -> Result<HashMap<String, String>, std::io::Error> {
    let tokens: HashMap<String, String> = HashMap::new();
    let reader = BufReader::new(fp);
    let tre = Regex::new(REGEX_IDENTIFIER).unwrap();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let line = line.trim();
        if tre.is_match(&line) {
            let captured = tre.find(&line).unwrap();
            println!("captured: {:?}", &line[captured.start()..captured.end()]);
        }
    }
    Ok(tokens)
}
