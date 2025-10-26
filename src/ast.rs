#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Text(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Var(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    FuncCall(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Make(String, Option<Expr>, Vec<Stmt>), // name, expr, body for func
    Change(String, Expr),
    Say(Expr),
    If(Expr, Vec<Stmt>),
    Repeat(Expr, Vec<Stmt>),
    ExprStmt(Expr), // for func calls
}
