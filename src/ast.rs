#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    Text(String),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Value(Value),
    Var(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    FuncCall(String),
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Make(String, Option<Expr>, Vec<Stmt>),
    Change(String, Expr),
    Say(Expr),
    If(Expr, Vec<Stmt>),
    Repeat(Expr, Vec<Stmt>),
    ExprStmt(Expr),
}
