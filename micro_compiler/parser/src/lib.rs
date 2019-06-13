use scanner::{Token, TokenType};
use std::collections::VecDeque;

pub struct Program {
    pub id: String,
    pub pgm_body: Option<String>,
}

pub struct GlobalString {
    pub id: String,
    pub s: String,
}

pub fn parse_file(tokens: VecDeque<Token>) {
    let mut tok_iter = tokens.iter().peekable();

    while let Some(tok) = tok_iter.next() {
        match tok.token_type {
            TokenType::KEYWORD => println!("keyword"),
            TokenType::IDENTIFIER => println!("identifier"),
            TokenType::FLOATLITERAL => println!("floatliteral"),
            TokenType::INTLITERAL => println!("intliteral"),
            TokenType::STRINGLITERAL => println!("stringliteral"),
            TokenType::OPERATOR => println!("operator"),
            _ => continue,
        }

        if let Some(peeked) = tok_iter.peek() {
            print!("\t");
            match peeked.token_type {
                TokenType::KEYWORD => println!("keyword"),
                TokenType::IDENTIFIER => println!("identifier"),
                TokenType::FLOATLITERAL => println!("floatliteral"),
                TokenType::INTLITERAL => println!("intliteral"),
                TokenType::STRINGLITERAL => println!("stringliteral"),
                TokenType::OPERATOR => println!("operator"),
                _ => continue,
            }
        }
    }
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
