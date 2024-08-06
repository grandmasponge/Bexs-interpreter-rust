use std::ops::Deref;

use crate::expr::Expr;
use crate::expr::ExprLiteral;
use crate::Token;
use crate::TokenType;
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

    pub fn EvaulateUnary(opcode: &Token, unary: &Box<Expr>) {
        match opcode._string.as_str() {
            "-" => {
                let expr = unary.deref();
                if let Expr::Literal(ExprLiteral::Number(num)) = expr {
                    let f32 = num.parse::<f32>().unwrap();
                    println!("-{f32}");
                } else {
                    println!("error!");
                }
            }
            "!" => {
                let expr = unary.deref();
                if let Expr::Literal(ExprLiteral::Bool(bang)) = expr {
                    if *bang {
                        println!("{}", !true);
                    } else {
                        println!("{}", !false);
                    }
                }
            }
            _ => unreachable!(),
        }
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
