use crate::expr::Expr;
use crate::expr::ExprLiteral;
pub struct Evaluator;

impl Evaluator {
    pub fn Evaluate(expr: &Expr) {
        match expr {
            Expr::Literal(v) => Self::EvaluateLiteral(&v),
            Expr::Grouping(expr) => Self::EvaluateGrouping(expr),
            _ => unreachable!(),
        }
    }
    pub fn EvaluateGrouping(grouping: &Box<Expr>) {
        Evaluator::Evaluate(grouping)
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
}
