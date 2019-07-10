use scanner::{Token, TokenType};
// use std::collections::VecDeque;

pub enum VarType {
    FLOAT,
    INT,
    VOID,
}

pub struct Program<'a> {
    pub id: String,
    pub pgm_body: Option<&'a [Token]>,
}

pub struct GlobalString {
    pub id: Token,
    pub st: String,
}

pub struct Variable<'a> {
    pub var_type: VarType,
    pub id: Token,
    pub id_tail: Option<&'a [Token]>,
}

pub struct FunctionParamList<'a> {
    pub var_type: VarType,
    pub id: Token,
    pub param_tail: Option<&'a [Token]>,
}

pub struct FunctionDeclaration<'a> {
    pub return_type: VarType,
    pub id: Token,
    pub parameters: FunctionParamList<'a>,
    pub func_body: Option<&'a [Token]>,
}

pub fn parse_file(tokens: Vec<Token>) {
    let mut tok_iter = tokens.iter().peekable();

    while let Some(tok) = tok_iter.next() {
        match tok.token_type {
            TokenType::KEYWORD => {
                // TODO build structs to hold keyword types
                println!("keyword");
            }
            TokenType::IDENTIFIER => {
                // TODO
                println!("identifier");
            }
            TokenType::FLOATLITERAL => {
                println!("floatliteral");
            }
            TokenType::INTLITERAL => {
                println!("intliteral");
            }
            TokenType::STRINGLITERAL => {
                println!("stringliteral");
            }
            TokenType::OPERATOR => {
                println!("operator");
            }
            _ => unreachable!(),
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
