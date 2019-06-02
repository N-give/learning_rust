#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate num_derive;

use fancy_regex::{Captures, Regex};
// use num;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;

static IDENTIFIER_PATTERN: &str = r"[a-zA-Z]{1}\w*";
static INTLITERAL_PATTERN: &str = r"\d+";
static FLOATLITER_PATTERN: &str = r"\d*\.\d+";
static STRINGLITERAL_PATTERN: &str = r#"".*""#;
static COMMENT_PATTERN: &str = r"--.*";
static OPERATORS_PATTERN: &str = r"\+|(?<!-)-(?!-)|\*|/|\(|\)|;|,|!=|:=|<=|<|>=|>|=";
static KEYWORDS_PATTERN: &str = r"PROGRAM|BEGIN|FUNCTION|READ|WRITE|ELSE|ENDIF|IF|WHILE|ENDWHILE|RETURN|INT|VOID|STRING|FLOAT|TRUE|FALSE|ENDFOR|FOR|CONTINUE|END|BREAK";

lazy_static! {
    static ref ALL_REGEX: Vec<Regex> = vec![
        Regex::new(KEYWORDS_PATTERN).unwrap(),
        Regex::new(IDENTIFIER_PATTERN).unwrap(),
        Regex::new(FLOATLITER_PATTERN).unwrap(),
        Regex::new(INTLITERAL_PATTERN).unwrap(),
        Regex::new(STRINGLITERAL_PATTERN).unwrap(),
        Regex::new(COMMENT_PATTERN).unwrap(),
        Regex::new(OPERATORS_PATTERN).unwrap(),
    ];
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Token {
        Token { token_type, value }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token Type: {:?}\nValue: {}",
            self.token_type, self.value
        )
    }
}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum TokenType {
    KEYWORD = 0,
    IDENTIFIER = 1,
    FLOATLITERAL = 2,
    INTLITERAL = 3,
    STRINGLITERAL = 4,
    COMMENT = 5,
    OPERATOR = 6,
}

pub fn scan_file(fp: std::fs::File) -> Result<VecDeque<Token>, std::io::Error> {
    let reader = BufReader::new(fp);
    let mut tokens = VecDeque::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let mut til = get_tokens(&line.trim())?;
        // println!("{:?}", til);
        if !til.is_empty() {
            tokens.append(&mut til);
        }
    }
    Ok(tokens)
}

fn get_tokens(line: &str) -> Result<VecDeque<Token>, std::io::Error> {
    let mut toks: VecDeque<Token> = VecDeque::new();
    // let mut end: usize = 0;

    let all_caps: Vec<Vec<Captures>> = line
        .split_whitespace()
        .map(|word| {
            ALL_REGEX
                .iter()
                .map(|re| re.captures(word).unwrap())
                .filter_map(|cap| cap)
                .collect()
        })
        .collect();

    for caps in all_caps.iter() {
        for cap in caps.iter() {
            for group in cap.iter() {
                println!("{:?}", group.unwrap());
            }
        }
    }
    // println!("{:?}", caps);
    /*
    while {
        // TODO
        // rework this section to use fancy regex instead of RegexSet
        let caps: Vec<Captures> = ALL_REGEX
            .iter()
            .map(|re| re.captures(&line).unwrap())
            .filter_map(|cap| cap)
            .collect();

        for cap in caps.iter() {
            for g in cap.iter() {
                println!("{:?}", g.unwrap());
            }
        }
        // end < line.len()
        false
    } {}
    */
    Ok(toks)
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
