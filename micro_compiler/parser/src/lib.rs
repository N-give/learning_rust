use scanner::Token;
use scanner::TokenType;
// use std::collections::VecDeque;

pub enum VarType {
    FLOAT,
    INT,
    VOID,
}

#[derive(Debug)]
pub enum StatementType {
    ASSIGN, // id := expr
    READ, // READ ( id_list );
    WRITE, // WRITE ( id_list );
    RETURN, // RETURN expr;
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Statement {
    pub stmt_type: StatementType,
    pub stmt: Vec<Token>,
}

impl Statement {
    pub fn new(items: Vec<Token>) -> Statement {
        Statement {
            stmt_type: StatementType::ASSIGN,
            stmt: items,
        }
    }
}

pub fn parse_file(mut tokens: Vec<Token>) {
    while !tokens.is_empty() {
        match tokens[0].token_type {
            TokenType::KEYWORD => handle_keyword(&mut tokens),
            TokenType::IDENTIFIER => unimplemented!(),
            TokenType::OPERATOR => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

fn handle_keyword(tokens: &mut Vec<Token>) {
    match tokens[0].value {
        "PROGRAM" => unimplemented!(),
        "BEGIN" => unimplemented!(),
        "FUNCTION" => unimplemented!(),
        "READ" => unimplemented!(),
        "WRITE" => unimplemented!(),
        "ELSE" => unimplemented!(),
        "ENDIF" => unimplemented!(),
        "IF" => unimplemented!(),
        "WHILE" => unimplemented!(),
        "ENDWHILE" => unimplemented!(),
        "RETURN" => unimplemented!(),
        "INT" => unimplemented!(),
        "VOID" => unimplemented!(),
        "STRING" => unimplemented!(),
        "FLOAT" => unimplemented!(),
        "TRUE" => unimplemented!(),
        "FALSE" => unimplemented!(),
        "ENDFOR" => unimplemented!(),
        "FOR" => unimplemented!(),
        "CONTINUE" => unimplemented!(),
        "END" => unimplemented!(),
        "BREAK" => unimplemented!(),
        _ => unimplemented!(),
    }
}

pub fn get_program(tokens: &mut Vec<Token>) -> Program {
    Program {
        id: tokens.remove(0).value,
        pgm_body: None,
    }
}

pub fn get_basic_statement(tokens: &mut Vec<Token>) -> Option<Statement> {
    if let Some(pos) = tokens.iter().position(|t| t.value == ";") {
        let stmt = tokens.drain(..=pos);
        Some(Statement::new(stmt.collect()))
    } else {
        None
    }
}
