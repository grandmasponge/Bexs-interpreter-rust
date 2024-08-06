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
    match literal {
        ExprLiteral::Bool(bool) => println!("{bool}"),
        ExprLiteral::Nil => println!("nil"),
        ExprLiteral::Number(num) => {
            let intergerliteral = num.parse::<f32>().unwrap();
            println!("{intergerliteral}");
        }
        ExprLiteral::String(str) => {
            println!("{str}");
        }
    }
}
