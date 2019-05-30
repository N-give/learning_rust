extern crate num;
extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;

use regex::{Regex, RegexSet};
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;

static IDENTIFIER_PATTERN: &str = r"[a-zA-Z]{1}[a-zA-Z0-9]*";
static INTLITERAL_PATTERN: &str = r"\d+";
static FLOATLITER_PATTERN: &str = r"\d*\.\d+";
static STRINGLITERAL_PATTERN: &str = r#"".*""#;
static COMMENT_PATTERN: &str = r"--.*";
static OPERATORS_PATTERN: &str = r"[+]|-{1}|[*]|/|\(|\)|;|,|!=|:=|<=|<|>=|>|=";
static KEYWORDS_PATTERN: &str = r"PROGRAM|BEGIN|FUNCTION|READ|WRITE|ELSE|ENDIF|IF|WHILE|ENDWHILE|RETURN|INT|VOID|STRING|FLOAT|TRUE|FALSE|ENDFOR|FOR|CONTINUE|END|BREAK";

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
        write!(f, "Token Type: {:?}\nValue: {}", self.token_type, self.value)
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
        ]).unwrap();
    }

    let mut toks: VecDeque<Token> = VecDeque::new();
    let mut end: usize = 0;

    while {
        let matched = REGEX_SET.matches(&line[end..]);
        if matched.matched_any() {
            // TODO
            // very un-optimized
            // should cache results from here and work through those before
            // testing again
            let (t, m) = matched
                .into_iter()
                .map(|ri| (ri, ALL_REGEX[ri].find(&line[end..]).unwrap()))
                .min_by(|x, y| x.1.start().cmp(&y.1.start()))
                .unwrap();
            let t = num::FromPrimitive::from_usize(t).unwrap();
            if t != TokenType::COMMENT {
                toks.push_back(Token::new(
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
