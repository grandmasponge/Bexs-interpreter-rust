pub enum Expr {
    Bool(bool),
    Nil,
    Number(String),
    String,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::String => write!(f, "not implimented"),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(num) => write!(f, "{num}"),
        }
    }
}
