#[allow(unused_variables)]
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

static REGEX_IDENTIFIER: &str = r"\w[a-zA-Z0-9]+";
static REGEX_INTLITERAL: &str = r"\d+";
static REGEX_FLOATLITER: &str = r"\d*\.\d+";
static REGEX_STRINGLITERAL: &str = r#"".*""#;
static REGEX_COMMENT: &str = r"--.*$";
// TODO Failes to compile regex
static REGEX_OPERATORS: &str = r":=|[+]|-|[*]|/|=|!=|<|>|(|)|;|,|<=|>=";
static REGEX_KEYWORDS: &str = r"PROGRAM|BEGIN|END|FUNCTION|READ|WRITE|IF|ELSE|ENDIF|WHILE|ENDWHILE|RETURN|INT|VOID|STRING|FLOAT|TRUE|FALSE|FOR|ENDFOR|CONTINUE|BREAK";

pub fn scan_file(fp: std::fs::File) -> Result<HashMap<String, String>, std::io::Error> {
    let tokens: HashMap<String, String> = HashMap::new();
    let reader = BufReader::new(fp);
    // TODO: need to test for multiple matches on a line
    let tre = Regex::new(REGEX_IDENTIFIER).unwrap();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let line = line.trim();
        // TODO
        if let Some(cap) = tre.find(&line) {
            println!("captured: {:?}", &line[cap.start()..cap.end()]);
        }
        /*
        if tre.is_match(&line) {
            let captured = tre.find(&line).unwrap();
            println!("captured: {:?}", &line[captured.start()..captured.end()]);
        }
        */
    }
    Ok(tokens)
}
