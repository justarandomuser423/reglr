use crate::lexer::Token;
use crate::ast::{Expr, Stmt, Value};
use logos::Lexer;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    slices: Vec<&'a str>,
    pos: usize,
    text: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: &mut Lexer<'a, Token>) -> Self {
        let tokens: Vec<Token> = lex.clone().collect();
        let mut text = Vec::new();
        lex.rewind();
        while let Some(_) = lex.next() {
            text.push(lex.slice());
        }
        Parser { tokens, slices: vec![], pos: 0, text }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn eat(&mut self, expected: &Token) -> bool {
        if let Some(t) = self.current() {
            if std::mem::discriminant(t) == std::mem::discriminant(expected) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.pos < self.tokens.len() {
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            } else {
                self.advance();
            }
        }

        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current()? {
            Token::Make => {
                self.advance();
                if let Some(Token::Identifier) = self.current() {
                    let name = self.text[self.pos].to_string();
                    self.advance();

                    match self.current() {
                        Some(Token::Be) => {
                            self.advance();
                            let expr = self.parse_expr();
                            Some(Stmt::Make(name, Some(expr), vec![]))
                        }
                        Some(Token::Do) => {
                            self.advance();
                            let body = self.parse_block();
                            Some(Stmt::Make(name, None, body))
                        }
                        _ => None,
                    }
                } else { None }
            }
            Token::Change => {
                self.advance();
                if let Some(Token::Identifier) = self.current() {
                    let name = self.text[self.pos].to_string();
                    self.advance();
                    if let Some(Token::Identifier) = self.current() {
                        self.advance(); // skip 'to'
                        let expr = self.parse_expr();
                        Some(Stmt::Change(name, expr))
                    } else { None }
                } else { None }
            }
            Token::Say => {
                self.advance();
                let expr = self.parse_expr();
                Some(Stmt::Say(expr))
            }
            Token::If => {
                self.advance();
                let cond = self.parse_expr();
                let body = self.parse_block();
                Some(Stmt::If(cond, body))
            }
            Token::Repeat => {
                self.advance();
                let times = self.parse_expr();
                if self.eat(&Token::Times) {
                    let body = self.parse_block();
                    Some(Stmt::Repeat(times, body))
                } else { None }
            }
            Token::Identifier => {
                let name = self.text[self.pos].to_string();
                self.advance();
                Some(Stmt::ExprStmt(Expr::FuncCall(name)))
            }
            _ => None,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        // naive: parse next statements until end or dedent (for simplicity we parse all remaining)
        let mut stmts = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            } else {
                break;
            }
        }
        stmts
    }

    // recursive descent math parsing
    fn parse_expr(&mut self) -> Expr {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Expr {
        let mut node = self.parse_mul_div();
        while let Some(token) = self.current() {
            match token {
                Token::Plus => { self.advance(); node = Expr::BinaryOp(Box::new(node), "+".into(), Box::new(self.parse_mul_div())); }
                Token::Minus => { self.advance(); node = Expr::BinaryOp(Box::new(node), "-".into(), Box::new(self.parse_mul_div())); }
                _ => break,
            }
        }
        node
    }

    fn parse_mul_div(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while let Some(token) = self.current() {
            match token {
                Token::Star => { self.advance(); node = Expr::BinaryOp(Box::new(node), "*".into(), Box::new(self.parse_primary())); }
                Token::Slash => { self.advance(); node = Expr::BinaryOp(Box::new(node), "/".into(), Box::new(self.parse_primary())); }
                Token::Percent => { self.advance(); node = Expr::BinaryOp(Box::new(node), "%".into(), Box::new(self.parse_primary())); }
                _ => break,
            }
        }
        node
    }

    fn parse_primary(&mut self) -> Expr {
        if let Some(t) = self.current() {
            match t {
                Token::Number => { let val = self.text[self.pos].parse::<i64>().unwrap_or(0); self.advance(); Expr::Value(Value::Number(val)) }
                Token::String => { let s = self.text[self.pos].trim_matches('"').to_string(); self.advance(); Expr::Value(Value::Text(s)) }
                Token::Identifier => { let name = self.text[self.pos].to_string(); self.advance(); Expr::Var(name) }
                Token::LParen => {
                    self.advance();
                    let node = self.parse_expr();
                    if let Some(Token::RParen) = self.current() { self.advance(); }
                    node
                }
                _ => { self.advance(); Expr::Value(Value::Number(0)) }
            }
        } else {
            Expr::Value(Value::Number(0))
        }
    }
}
