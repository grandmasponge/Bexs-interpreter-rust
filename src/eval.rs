use crate::expr::{Expr, ExprLiteral};

pub struct Evaluator {
    Expression: Expr,
}

impl Evaluator {
    pub fn new(Expression: Expr) -> Self {
        Self { Expression }
    }
    pub fn Evaluate(&self) -> i32 {
        match &self.Expression {
            Expr::Literal(v) => EvaluateLiteral(&v),
            _ => unreachable!(),
        }
        0
    }
}

pub fn EvaluateLiteral(literal: &ExprLiteral) {
    //for now just print the literal later we match and shii
    println!("{literal}");
}
