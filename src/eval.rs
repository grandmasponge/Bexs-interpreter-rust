use std::collections::btree_map::Values;
use std::ops::Deref;

use crate::expr::Expr;
use crate::expr::ExprLiteral;
use crate::Token;
use crate::TokenType;
pub struct Evaluator;

pub enum Value {
    String(String),
    Number(f32),
    Nil,
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(fmt, "{}", s),
            Value::Nil => write!(fmt, "nil"),
            Value::Number(n) => write!(fmt, "{}", n),
            Value::Bool(b) => write!(fmt, "{}", b),
        }
    }
}

impl Evaluator {
    pub fn Evaluate(expr: &Expr) -> Value {
        match expr {
            Expr::Literal(v) => Self::EvaluateLiteral(&v),
            Expr::Grouping(expr) => Self::Evaluate(expr),
            Expr::Unary(op, expr) => Self::EvalUnary(op, expr),
            Expr::Binary(op, left, right) => Self::EvalBinary(op, left, right),
            _ => unreachable!(),
        }
    }

    pub fn EvalBinary(op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> Value {
        let left = Self::Evaluate(left);
        let right = Self::Evaluate(right);

        match op._string.as_str() {
            "*" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Number(lhs * rhs);
                } else {
                    Value::Nil
                }
            }
            "/" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Number(lhs / rhs);
                } else {
                    Value::Nil
                }
            }
            "+" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Number(lhs + rhs);
                } else {
                    if let Value::String(lhs) = left {
                        let mut rhs = String::new();
                        if let Value::String(r) = right {
                            rhs = r;
                        }
                        Value::String(format!("{}{}", lhs, rhs))
                    } else {
                        Value::Nil
                    }
                }
            }
            "-" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Number(lhs - rhs);
                } else {
                    Value::Nil
                }
            }
            "<" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs < rhs));
                } else {
                    Value::Nil
                }
            }
            ">" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs > rhs));
                } else {
                    Value::Nil
                }
            }
            ">=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs >= rhs));
                } else {
                    Value::Nil
                }
            }
            "<=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs <= rhs));
                } else {
                    Value::Nil
                }
            }
            "==" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs == rhs));
                } else {
                    Value::Nil
                }
            }
            "!=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    }
                    return Value::Bool((lhs != rhs));
                } else {
                    Value::Nil
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn EvalUnary(op: &Token, expr: &Box<Expr>) -> Value {
        let right = Self::Evaluate(expr);
        match op._string.as_str() {
            "-" => {
                if let Value::Number(n) = right {
                    Value::Number(-n)
                } else {
                    Value::Nil
                }
            }
            "!" => match right {
                Value::Nil => Value::Bool(true),
                Value::Bool(b) => {
                    if b {
                        Value::Bool(false)
                    } else {
                        Value::Bool(true)
                    }
                }
                Value::Number(n) => Value::Bool(n == 0.0),
                Value::String(s) => Value::Bool(s.is_empty()),
            },
            _ => unreachable!(),
        }
    }

    pub fn EvaluateLiteral(literal: &ExprLiteral) -> Value {
        match literal {
            ExprLiteral::Bool(truthy) => Value::Bool(*truthy),
            ExprLiteral::String(Stringy) => Value::String(Stringy.to_owned()),
            ExprLiteral::Number(numy) => {
                let f32 = numy.parse::<f32>().unwrap();
                Value::Number(f32)
            }
            ExprLiteral::Nil => Value::Nil,
        }
    }
}
