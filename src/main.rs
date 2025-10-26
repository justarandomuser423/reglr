mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use logos::Logos;
use lexer::Token;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let path = std::env::args().nth(1).expect("need script path");
    let script = fs::read_to_string(path).expect("cannot read file");

    let mut texts = vec![];
    let mut tokens = vec![];

    let mut lex = Token::lexer(&script);
    while let Some(result) = lex.next() {
        match result {
            Ok(tok) => {
                tokens.push(tok.clone());
                match tok {
                    Token::Number | Token::Identifier | Token::Text => texts.push(lex.slice()),
                    _ => texts.push(""),
                }
            }
            Err(_) => {}
        }
    }

    let mut parser = Parser::new(tokens, texts);
    let stmts = parser.parse();

    let mut interp = Interpreter::new();
    interp.run(&stmts);
}
