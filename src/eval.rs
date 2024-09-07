use core::fmt;
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
#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
    //for now set line to juss 1
    line: u32,
    pub exit: i32,
}

impl RuntimeError {
    fn new(msg: String) -> Self {
        Self {
            msg,
            line: 1,
            exit: 70,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}\n[line {}]", self.msg, self.line)
    }
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
    pub fn Evaluate(expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(v) => Ok(Self::EvaluateLiteral(&v)),
            Expr::Grouping(expr) => Self::Evaluate(expr),
            Expr::Unary(op, expr) => Self::EvalUnary(op, expr),
            Expr::Binary(op, left, right) => Self::EvalBinary(op, left, right),
            _ => unreachable!(),
        }
    }

    pub fn EvalBinary(
        op: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Value, RuntimeError> {
        let left = Self::Evaluate(left)?;
        let right = Self::Evaluate(right)?;

        match op._string.as_str() {
            "*" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Number(lhs * rhs));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            "/" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Number(lhs / rhs));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            "+" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Number(lhs + rhs));
                } else {
                    if let Value::String(lhs) = left {
                        let mut rhs = String::new();
                        if let Value::String(r) = right {
                            rhs = r;
                        } else {
                            return Err(RuntimeError::new(String::from(
                                "Operands must be numbers.",
                            )));
                        }
                        Ok(Value::String(format!("{}{}", lhs, rhs)))
                    } else {
                        Err(RuntimeError::new(String::from("Operands must be numbers.")))
                    }
                }
            }
            "-" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Number(lhs - rhs));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            "<" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Bool((lhs < rhs)));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            ">" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Bool((lhs > rhs)));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            ">=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Bool((lhs >= rhs)));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            "<=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(String::from("Operands must be numbers.")));
                    }
                    return Ok(Value::Bool((lhs <= rhs)));
                } else {
                    Err(RuntimeError::new(String::from("Operands must be numbers.")))
                }
            }
            "==" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Ok(Value::Bool(false));
                    }
                    return Ok(Value::Bool((lhs == rhs)));
                } else if let Value::Bool(lhs) = left {
                    let mut rhs = false;
                    if let Value::Bool(r) = right {
                        rhs = r;
                    } else {
                        return Ok(Value::Bool(false));
                    }
                    return Ok(Value::Bool((lhs == rhs)));
                } else {
                    if let Value::String(lhs) = left {
                        let mut rhs = String::new();
                        if let Value::String(r) = right {
                            rhs = r;
                        } else {
                            return Ok(Value::Bool(false));
                        }
                        Ok(Value::Bool((lhs == rhs)))
                    } else {
                        Err(RuntimeError::new(String::from("Operands must be numbers.")))
                    }
                }
            }
            "!=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Ok(Value::Bool(false));
                    }
                    return Ok(Value::Bool((lhs != rhs)));
                }
                if let Value::Bool(lhs) = left {
                    let mut rhs = false;
                    if let Value::Bool(r) = right {
                        rhs = r;
                    } else {
                        return Ok(Value::Bool(false));
                    }
                    return Ok(Value::Bool((lhs != rhs)));
                } else {
                    if let Value::String(lhs) = left {
                        let mut rhs = String::new();
                        if let Value::String(r) = right {
                            rhs = r;
                        } else {
                            return Ok(Value::Bool(false));
                        }
                        Ok(Value::Bool((lhs != rhs)))
                    } else {
                        Err(RuntimeError::new(String::from("Operands must be numbers.")))
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn EvalUnary(op: &Token, expr: &Box<Expr>) -> Result<Value, RuntimeError> {
        let right = Self::Evaluate(expr)?;
        match op._string.as_str() {
            "-" => {
                if let Value::Number(n) = right {
                    Ok(Value::Number(-n))
                } else {
                    Err(RuntimeError::new(String::from(
                        "Operand must be an number.",
                    )))
                }
            }
            "!" => match right {
                Value::Nil => Ok(Value::Bool(true)),
                Value::Bool(b) => {
                    if b {
                        Ok(Value::Bool(false))
                    } else {
                        Ok(Value::Bool(true))
                    }
                }
                Value::Number(n) => Ok(Value::Bool(n == 0.0)),
                Value::String(s) => Ok(Value::Bool(s.is_empty())),
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
