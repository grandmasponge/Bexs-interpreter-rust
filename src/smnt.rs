use crate::expr::Expr;

struct StatementError {}

pub enum Statment {
    ExprStmt(Expr),
    PrintStmt(Expr),
}
