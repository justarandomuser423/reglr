use crate::ast::*;
use std::collections::HashMap;

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
            Stmt::Say(expr) => {
                let val = self.eval(expr);
                match val {
                    Value::Number(n) => println!("{}", n),
                    Value::Text(s) => println!("{}", s),
                }
            }
            Stmt::If(cond, body) => {
                let val = self.eval(cond);
                if matches!(val, Value::Number(n) if n != 0) {
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
                if let Expr::FuncCall(name) = expr {
                    if let Some(body) = self.funcs.get(name).cloned() {
                        self.run(&body);
                    }
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
        }
    }
}
