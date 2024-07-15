pub enum Expr {
    Bool(bool),
    Nil,
    Number(String),
    String(String),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(num) => write!(f, "{num}"),
            Expr::String(s) => write!(f, "{s}"),
        }
    }
}
