use crate::expr::Expr;
use crate::Token;
use crate::TokenType;

pub struct Parser {
    pub expressions: Vec<Expr>,
}
impl Parser {
    pub fn parse_from_tokens(tokens: Vec<Token>) -> Self {
        let mut expr = Vec::new();
        for token in tokens {
            match token._type {
                TokenType::True => expr.push(Expr::Bool(true)),
                TokenType::Nil => expr.push(Expr::Nil),
                TokenType::False => expr.push(Expr::Bool(false)),
                TokenType::Number => expr.push(Expr::Number(token._string.clone())),
                _ => {}
            }
        }

        Self { expressions: expr }
    }
}
