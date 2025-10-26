use crate::lexer::Token;
use crate::ast::{Expr, Stmt, Value};
use logos::Lexer;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    slices: Vec<&'a str>,
    pos: usize,
    text: Vec<&'a str>, // slice text for identifiers/numbers
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

        while let Some(token) = self.current() {
            match token {
                Token::Make => {
                    self.advance();
                    if let Some(Token::Identifier) = self.current() {
                        let name = self.text[self.pos].to_string();
                        self.advance();

                        if let Some(Token::Be) = self.current() {
                            self.advance();
                            let expr = self.parse_expr();
                            stmts.push(Stmt::Make(name, expr));
                        } else if let Some(Token::Do) = self.current() {
                            // placeholder for function body
                            self.advance();
                            // ignore body for now
                        }
                    }
                }
                Token::Change => {
                    self.advance();
                    if let Some(Token::Identifier) = self.current() {
                        let name = self.text[self.pos].to_string();
                        self.advance();
                        if self.eat(&Token::Identifier) { // expecting 'to'
                            let expr = self.parse_expr();
                            stmts.push(Stmt::Change(name, expr));
                        }
                    }
                }
                Token::Say => {
                    self.advance();
                    let expr = self.parse_expr();
                    stmts.push(Stmt::Say(expr));
                }
                _ => self.advance(), // skip unknown
            }
        }

        stmts
    }

    impl<'a> Parser<'a> {
    // parse full expression
    fn parse_expr(&mut self) -> Expr {
        self.parse_add_sub()
    }

    // lowest precedence + and -
    fn parse_add_sub(&mut self) -> Expr {
        let mut node = self.parse_mul_div();
        while let Some(token) = self.current() {
            match token {
                Token::Plus => {
                    self.advance();
                    node = Expr::BinaryOp(Box::new(node), "+".to_string(), Box::new(self.parse_mul_div()));
                }
                Token::Minus => {
                    self.advance();
                    node = Expr::BinaryOp(Box::new(node), "-".to_string(), Box::new(self.parse_mul_div()));
                }
                _ => break,
            }
        }
        node
    }

    // higher precedence * / %
    fn parse_mul_div(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while let Some(token) = self.current() {
            match token {
                Token::Star => {
                    self.advance();
                    node = Expr::BinaryOp(Box::new(node), "*".to_string(), Box::new(self.parse_primary()));
                }
                Token::Slash => {
                    self.advance();
                    node = Expr::BinaryOp(Box::new(node), "/".to_string(), Box::new(self.parse_primary()));
                }
                Token::Percent => {
                    self.advance();
                    node = Expr::BinaryOp(Box::new(node), "%".to_string(), Box::new(self.parse_primary()));
                }
                _ => break,
            }
        }
        node
    }

    fn parse_primary(&mut self) -> Expr {
        if let Some(t) = self.current() {
            match t {
                Token::Number => {
                    let val = self.text[self.pos].parse::<i64>().unwrap_or(0);
                    self.advance();
                    Expr::Value(Value::Number(val))
                }
                Token::String => {
                    let s = self.text[self.pos].trim_matches('"').to_string();
                    self.advance();
                    Expr::Value(Value::Text(s))
                }
                Token::Identifier => {
                    let name = self.text[self.pos].to_string();
                    self.advance();
                    Expr::Var(name)
                }
                Token::LParen => {
                    self.advance();
                    let node = self.parse_expr();
                    if let Some(Token::RParen) = self.current() {
                        self.advance();
                    }
                    node
                }
                _ => {
                    self.advance();
                    Expr::Value(Value::Number(0))
                }
            }
        } else {
            Expr::Value(Value::Number(0))
        }
    }
}
