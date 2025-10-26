use crate::lexer::Token;
use crate::ast::*;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    text: Vec<&'a str>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, text: Vec<&'a str>) -> Self {
        Parser { tokens, text, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn current_text(&self) -> Option<&str> {
        self.text.get(self.pos).copied()
    }

    fn eat(&mut self, tok: Token) -> bool {
        if Some(&tok) == self.current() {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.pos += 1;
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current()? {
            Token::Make => {
                self.pos += 1;
                let name = self.current_text()?.to_string();
                self.pos += 1;
                let mut expr = None;
                if self.eat(Token::Be) {
                    expr = Some(self.parse_expr());
                }
                let mut body = vec![];
                if self.eat(Token::Do) {
                    while self.pos < self.tokens.len() {
                        if matches!(self.current(), Some(Token::Identifier)) { break; }
                        if let Some(s) = self.parse_stmt() { body.push(s); }
                        else { self.pos += 1; }
                    }
                }
                Some(Stmt::Make(name, expr, body))
            }
            Token::Change => {
                self.pos += 1;
                let name = self.current_text()?.to_string();
                self.pos += 1;
                self.eat(Token::To);
                let expr = self.parse_expr();
                Some(Stmt::Change(name, expr))
            }
            Token::Say => {
                self.pos += 1;
                let expr = self.parse_expr();
                Some(Stmt::Say(expr))
            }
            Token::If => {
                self.pos += 1;
                let cond = self.parse_expr();
                self.eat(Token::Do);
                let mut body = vec![];
                while self.pos < self.tokens.len() {
                    if matches!(self.current(), Some(Token::Identifier)) { break; }
                    if let Some(s) = self.parse_stmt() { body.push(s); }
                    else { self.pos += 1; }
                }
                Some(Stmt::If(cond, body))
            }
            Token::Repeat => {
                self.pos += 1;
                let times = self.parse_expr();
                self.eat(Token::Times);
                let mut body = vec![];
                while self.pos < self.tokens.len() {
                    if matches!(self.current(), Some(Token::Identifier)) { break; }
                    if let Some(s) = self.parse_stmt() { body.push(s); }
                    else { self.pos += 1; }
                }
                Some(Stmt::Repeat(times, body))
            }
            Token::Identifier => {
                let name = self.current_text()?.to_string();
                self.pos += 1;
                Some(Stmt::ExprStmt(Expr::FuncCall(name)))
            }
            _ => None,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        if let Some(token) = self.current() {
            match token {
                Token::Number => {
                    let val = self.current_text().unwrap_or("0").parse::<i64>().unwrap_or(0);
                    self.pos += 1;
                    Expr::Value(Value::Number(val))
                }
                Token::Identifier => {
                    let name = self.current_text().unwrap_or("").to_string();
                    self.pos += 1;
                    Expr::Var(name)
                }
                Token::Text => {
                    let val = self.current_text().unwrap_or("").to_string();
                    self.pos += 1;
                    Expr::Value(Value::Text(val))
                }
                _ => {
                    self.pos += 1;
                    Expr::Value(Value::Number(0))
                }
            }
        } else {
            Expr::Value(Value::Number(0))
        }
    }
}
