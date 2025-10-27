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
                let mut exprs = vec![];
                exprs.push(self.parse_expr());
                Some(Stmt::Say(exprs))
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
	    Token::Forever => {
    		self.pos += 1;
    		let mut body = vec![];
    		self.eat(Token::Do);
    		while self.pos < self.tokens.len() {
        		if matches!(self.current(), Some(Token::Identifier)) { break; }
        		if let Some(s) = self.parse_stmt() { body.push(s); }
        		else { self.pos += 1; }
    		}
    		Some(Stmt::Forever(body))
	    }
            Token::KeyPressed => {
                self.pos += 1;
                if let Some(Token::Text) = self.current() {
                    let key = self.current_text()?.to_string();
                    self.pos += 1;
                    Some(Stmt::ExprStmt(Expr::KeyPressed(key)))
                } else { None }
            }
            _ => None,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = match self.current() {
            Some(Token::Number) => {
                let val = self.current_text().unwrap_or("0").parse::<i64>().unwrap_or(0);
                self.pos += 1;
                Expr::Value(Value::Number(val))
            }
            Some(Token::Text) => {
                let val = self.current_text().unwrap_or("").to_string();
                self.pos += 1;
                Expr::Value(Value::Text(val))
            }
            Some(Token::Identifier) => {
                let name = self.current_text().unwrap_or("").to_string();
                self.pos += 1;
                Expr::Var(name)
            }
            Some(Token::KeyPressed) => {
                self.pos += 1;
                if let Some(Token::Text) = self.current() {
                    let key = self.current_text().unwrap_or("").to_string();
                    self.pos += 1;
                    Expr::KeyPressed(key)
                } else {
                    Expr::Value(Value::Number(0))
                }
            }
            _ => { self.pos += 1; Expr::Value(Value::Number(0)) }
        };

        while let Some(op_token) = self.current() {
            let op_str = match op_token {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Star => "*",
                Token::Slash => "/",
                Token::Percent => "%",
                _ => break,
            };
            self.pos += 1;
            let right = self.parse_expr();
            left = Expr::BinaryOp(Box::new(left), op_str.to_string(), Box::new(right));
        }

        left
    }
}
