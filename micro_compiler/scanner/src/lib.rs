#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;

static IDENTIFIER_PATTERN: &str = r"\w[a-zA-Z0-9]+";
static INTLITERAL_PATTERN: &str = r"\d+";
static FLOATLITER_PATTERN: &str = r"\d*\.\d+";
static STRINGLITERAL_PATTERN: &str = r#"".*""#;
static COMMENT_PATTERN: &str = r"--.*$";
// TODO Failes to compile regex
// static OPERATORS_PATTERN: &str = r":=|[+]|-|[*]|/|=|!=|<|>|(|)|;|,|<=|>=";
static KEYWORDS_PATTERN: &str = r"PROGRAM|BEGIN|END|FUNCTION|READ|WRITE|IF|ELSE|ENDIF|WHILE|ENDWHILE|RETURN|INT|VOID|STRING|FLOAT|TRUE|FALSE|FOR|ENDFOR|CONTINUE|BREAK";

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Token {
        Token { token_type, value }
    }
}

#[derive(Debug)]
pub enum TokenType {
    IDENTIFIER,
    INTLITERAL,
    FLOATLITERAL,
    STRINGLITERAL,
    COMMENT,
    KEYWORD,
    OPERATOR,
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
        static ref REGEX_IDENTIFIER: Regex = Regex::new(IDENTIFIER_PATTERN).unwrap();
        static ref REGEX_INTLITERAL: Regex = Regex::new(INTLITERAL_PATTERN).unwrap();
        static ref REGEX_FLOATLITER: Regex = Regex::new(FLOATLITER_PATTERN).unwrap();
        static ref REGEX_STRINGLITERAL: Regex = Regex::new(STRINGLITERAL_PATTERN).unwrap();
        static ref REGEX_COMMENT: Regex = Regex::new(COMMENT_PATTERN).unwrap();
        static ref REGEX_KEYWORDS: Regex = Regex::new(KEYWORDS_PATTERN).unwrap();
    }

    let mut toks: VecDeque<Token> = VecDeque::new();
    let parts: VecDeque<Token> = line
        .split_whitespace()
        .filter_map(|word| match word {
            'A'...'Z' => Some(Token::new(word.to_string(), TokenType::KEYWORD)),
            _ => {
                println!("nothing yet");
                None
            }
        })
        .collect();
    // let mut end: usize = 0;
    /*
    let mut end: usize = 0;
    while {
        if let Some(fnd) = REGEX_KEYWORDS.find(&line[end..]) {
            toks.push_back(Token::new(
                line[fnd.start()..fnd.end()].to_string(),
                TokenType::KEYWORD,
            ));
            end = fnd.end();
        } else if let Some(fnd) = REGEX_IDENTIFIER.find(&line[end..]) {
            toks.push_back(Token::new(
                line[fnd.start()..fnd.end()].to_string(),
                TokenType::IDENTIFIER,
            ));
            end = fnd.end();
        }
        if let Some(fnd) = REGEX_INTLITERAL.find(&line[end..]) {
            toks.push_back(Token::new(
                line[fnd.start()..fnd.end()].to_string(),
                TokenType::INTLITERAL,
            ));
        }
        end < line.len()
    } {}
    line
        .split_whitespace()
        .for_each
    */
    Ok(toks)
}
