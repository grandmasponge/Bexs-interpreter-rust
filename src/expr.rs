pub enum ExprLiteral {
    Number(String),
    String(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for ExprLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprLiteral::Number(num) => write!(f, "{num}"),
            ExprLiteral::String(str) => write!(f, "{str}"),
            ExprLiteral::Nil => write!(f, "nil"),
            ExprLiteral::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

pub enum Expr {
    Literal(ExprLiteral),
    Grouping(Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{literal}"),
            Expr::Grouping(expr) => write!(f, "(group {})", *expr),
        }
    }
}
#[derive(Debug)]
pub struct ExprError {
    msg: String,
    code: i32,
}

impl ExprError {
    pub fn new(msg: String, code: i32) -> Self {
        Self { msg, code }
    }
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}
