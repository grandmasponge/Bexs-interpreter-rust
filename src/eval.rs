use crate::expr::Expr;
use crate::expr::ExprLiteral;
use crate::Token;
pub struct Evaluator;

impl Evaluator {
    pub fn Evaluate(expr: &Expr) {
        match expr {
            Expr::Literal(v) => Self::EvaluateLiteral(&v),
            Expr::Grouping(expr) => Self::EvaluateGrouping(expr),
            Expr::Unary(opcode, expr) => Self::EvaulateUnary(opcode, expr),
            _ => unreachable!(),
        }
    }

    pub fn EvaulateUnary(opcode: Token, unary: &Box<Expr>) {
        print!("{}", opcode._string);
        Self::Evaluate(unary);
    }

    pub fn EvaluateGrouping(grouping: &Box<Expr>) {
        Self::Evaluate(grouping)
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
