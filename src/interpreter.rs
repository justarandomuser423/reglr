use crate::ast::*;
use std::collections::HashMap;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

pub struct Interpreter {
    env: HashMap<String, Value>,
    funcs: HashMap<String, Vec<Stmt>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: HashMap::new(), funcs: HashMap::new() }
    }

    pub fn run(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Make(name, expr_opt, body) => {
                if let Some(expr) = expr_opt {
                    let val = self.eval(expr);
                    self.env.insert(name.clone(), val);
                }
                if !body.is_empty() {
                    self.funcs.insert(name.clone(), body.clone());
                }
            }
            Stmt::Change(name, expr) => {
                let val = self.eval(expr);
                self.env.insert(name.clone(), val);
            }
            Stmt::Say(exprs) => {
                let mut output = String::new();
                for expr in exprs {
                    let val = self.eval(expr);
                    match val {
                        Value::Number(n) => output.push_str(&n.to_string()),
                        Value::Text(s) => output.push_str(&s),
                    }
                }
                println!("{}", output);
            }
            Stmt::If(cond, body) => {
                let val = self.eval(cond);
                if matches!(val, Value::Number(n) if n != 0) {
                    self.run(body);
                }
            }
            Stmt::Forever(body) => {
                loop {
                    if let Some(key) = Self::is_key_pressed() {
                        println!("{}", key); // prints only the key letter
                    }
                    self.run(body);
                }
            }
            Stmt::Repeat(times_expr, body) => {
                let times = match self.eval(times_expr) {
                    Value::Number(n) => n,
                    _ => 0,
                };
                for _ in 0..times {
                    self.run(body);
                }
            }
            Stmt::ExprStmt(expr) => {
                match expr {
                    Expr::FuncCall(name) => {
                        if let Some(body) = self.funcs.get(name).cloned() {
                            self.run(&body);
                        }
                    }
                    Expr::BinaryOp(_, _, _) => {
                        self.eval(expr);
                    }
                    Expr::KeyPressed(_) => {
                        self.eval(expr);
                    }
                    _ => {}
                }
            }
        }
    }

    fn eval(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Value(v) => v.clone(),
            Expr::Var(name) => self.env.get(name).cloned().unwrap_or(Value::Number(0)),
            Expr::BinaryOp(left, op, right) => {
                let l = self.eval(left);
                let r = self.eval(right);
                match (l, r) {
                    (Value::Number(a), Value::Number(b)) => match op.as_str() {
                        "+" => Value::Number(a + b),
                        "-" => Value::Number(a - b),
                        "*" => Value::Number(a * b),
                        "/" => Value::Number(if b != 0 { a / b } else { 0 }),
                        "%" => Value::Number(if b != 0 { a % b } else { 0 }),
                        _ => Value::Number(0),
                    },
                    _ => Value::Number(0),
                }
            }
            Expr::FuncCall(_) => Value::Number(0),
            Expr::KeyPressed(key) => {
                if key == "any" {
                    if let Some(k) = Self::is_key_pressed() {
                        Value::Text(k) // return the pressed key as text
                    } else {
                        Value::Text(String::new())
                    }
                } else {
                    let pressed = Self::is_key_pressed();
                    if let Some(k) = pressed {
                        if k == *key { Value::Number(1) } else { Value::Number(0) }
                    } else {
                        Value::Number(0)
                    }
                }
            }
        }
    }

    fn is_key_pressed() -> Option<String> {
    	if poll(Duration::from_millis(1)).unwrap_or(false) {
            if let Event::Key(k) = read().unwrap() {
            	let key = match k.code {
                    KeyCode::Char(c) => c.to_string(),
                    KeyCode::Enter => "enter".to_string(),
                    KeyCode::Esc => "escape".to_string(),
                    KeyCode::Tab => "tab".to_string(),
                    KeyCode::Backspace => "backspace".to_string(),
                    KeyCode::Left => "left".to_string(),
                    KeyCode::Right => "right".to_string(),
                    KeyCode::Up => "up".to_string(),
                    KeyCode::Down => "down".to_string(),
                    _ => "unknown".to_string(),
            	};
            	return Some(key);
            }
        }
        None
    }
}

