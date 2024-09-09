use crate::expr::Expr;

struct StatementError {}

#[derive(Debug, Clone)]
pub enum Statment {
    ExprStmt(Expr),
    PrintStmt(Expr),
    BlockStatment(Box<Vec<Statment>>),
    VarDec(Expr, Expr),
}
