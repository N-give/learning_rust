#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

use num;
use regex::{self, Regex, RegexSet};
// use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;

const IDENTIFIER_PATTERN: &str = r"[a-zA-Z]{1}[a-zA-Z0-9]*";
const INTLITERAL_PATTERN: &str = r"\d+";
const FLOATLITER_PATTERN: &str = r"\d*\.\d+";
const STRINGLITERAL_PATTERN: &str = r#"".*""#;
const COMMENT_PATTERN: &str = r"--.*";
const OPERATORS_PATTERN: &str = r"\+|-|\*|/|\(|\)|;|,|!=|:=|<=|<|>=|>|=";
const KEYWORDS_PATTERN: &str = r"PROGRAM|BEGIN|FUNCTION|READ|WRITE|ELSE|ENDIF|IF|WHILE|ENDWHILE|RETURN|INT|VOID|STRING|FLOAT|TRUE|FALSE|ENDFOR|FOR|CONTINUE|END|BREAK";

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
    static ref REGEX_SET: RegexSet = RegexSet::new(&[
        KEYWORDS_PATTERN,
        IDENTIFIER_PATTERN,
        FLOATLITER_PATTERN,
        INTLITERAL_PATTERN,
        STRINGLITERAL_PATTERN,
        COMMENT_PATTERN,
        OPERATORS_PATTERN,
    ])
    .unwrap();
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

pub fn scan_file(fp: std::fs::File) -> Result<Vec<Token>, std::io::Error> {
    let reader = BufReader::new(fp);
    let mut tokens = Vec::new();

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

fn get_tokens(line: &str) -> Result<Vec<Token>, std::io::Error> {
    let mut toks: Vec<Token> = Vec::new();
    let mut end: usize = 0;

    while {
        let matched = REGEX_SET.matches(&line[end..]);
        if matched.matched_any() {
            // TODO
            // not very optimized
            // should cache results from here and work through those before
            // testing again
            let (t, m) = matched
                .into_iter()
                .map(|ri| (ri, ALL_REGEX[ri].find(&line[end..]).unwrap()))
                .min_by(|x, y| x.1.start().cmp(&y.1.start()))
                .unwrap();
            let t = num::FromPrimitive::from_usize(t).unwrap();
            if t != TokenType::COMMENT {
                toks.push(Token::new(
                    line[(end + m.start())..(end + m.end())].trim().to_string(),
                    t,
                ));
            }
            end += m.end();
        }
        end < line.len()
    } {}
    Ok(toks)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_test() {
        let test_string = "WHILE (x1 > 1)";
        assert_eq!(ALL_REGEX[0].find(&test_string).unwrap().as_str(), "WHILE");
    }

    #[test]
    fn identifier_test() {
        let test_string = "WHILE (x1 > 1)";
        assert_eq!(ALL_REGEX[1].find(&test_string).unwrap().as_str(), "x1");
    }

    /*
    fn keyword_test() {
        let test_string = "WHILE (x1 > 1)";
        assert_eq!(ALL_REGEX[0].find(&test_string).unwrap().as_str(), "WHILE");
    }

    #[test]
    fn keyword_test() {
        let test_string = "WHILE (x1 > 1)";
        assert_eq!(ALL_REGEX[0].find(&test_string).unwrap().as_str(), "WHILE");
    }

    #[test]
    fn keyword_test() {
        let test_string = "WHILE (x1 > 1)";
        assert_eq!(ALL_REGEX[0].find(&test_string).unwrap().as_str(), "WHILE");
    }
    */
}
*/
