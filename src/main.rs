mod lexer;
mod parser;
mod interpreter;
mod ast;

use std::env;
use std::fs;
use lexer::Token;
use logos::Logos;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <script.reglr>", args[0]);
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .expect("failed to read file");

    let mut lex = Token::lexer(&source);
    let mut parser = Parser::new(&mut lex);
    let statements = parser.parse(); // currently placeholder
    let mut interp = Interpreter::new();
    interp.run(statements);
}
