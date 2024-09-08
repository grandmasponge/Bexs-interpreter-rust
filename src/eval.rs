use core::fmt;
use std::collections::HashMap;
use std::ops::Deref;

use crate::expr::Expr;
use crate::expr::ExprLiteral;
use crate::Token;

pub struct Evaluator {
    pub line: u32,
    pub symbols: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
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
    pub line: u32,
    pub exit: i32,
}

impl RuntimeError {
    pub fn new(msg: String, line: u32) -> Self {
        Self {
            msg,
            line,
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
    pub fn new() -> Self {
        Self {
            line: 0,
            symbols: HashMap::new(),
        }
    }
    pub fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(v) => Ok(Self::EvaluateLiteral(self, &v)?),
            Expr::Grouping(expr) => Self::evaluate(self, expr),
            Expr::Unary(op, expr) => Self::EvalUnary(self, op, expr),
            Expr::Binary(op, left, right) => Self::EvalBinary(self, op, left, right),
            _ => unreachable!(),
        }
    }

    pub fn EvalBinary(
        &self,
        op: &Token,
        left: &Box<Expr>,
        right: &Box<Expr>,
    ) -> Result<Value, RuntimeError> {
        let left = Self::evaluate(self, left)?;
        let right = Self::evaluate(self, right)?;

        match op._string.as_str() {
            "*" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Number(lhs * rhs));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            "/" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Number(lhs / rhs));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            "+" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Number(lhs + rhs));
                } else {
                    if let Value::String(lhs) = left {
                        let mut rhs = String::new();
                        if let Value::String(r) = right {
                            rhs = r;
                        } else {
                            return Err(RuntimeError::new(
                                String::from("Operands must be numbers."),
                                self.line,
                            ));
                        }
                        Ok(Value::String(format!("{}{}", lhs, rhs)))
                    } else {
                        Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ))
                    }
                }
            }
            "-" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Number(lhs - rhs));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            "<" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Bool((lhs < rhs)));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            ">" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Bool((lhs > rhs)));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            ">=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Bool((lhs >= rhs)));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
                }
            }
            "<=" => {
                if let Value::Number(lhs) = left {
                    let mut rhs: f32 = 0.;
                    if let Value::Number(r) = right {
                        rhs = r;
                    } else {
                        return Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ));
                    }
                    return Ok(Value::Bool((lhs <= rhs)));
                } else {
                    Err(RuntimeError::new(
                        String::from("Operands must be numbers."),
                        self.line,
                    ))
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
                        Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ))
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
                        Err(RuntimeError::new(
                            String::from("Operands must be numbers."),
                            self.line,
                        ))
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn EvalUnary(&self, op: &Token, expr: &Box<Expr>) -> Result<Value, RuntimeError> {
        let right = Self::evaluate(self, expr)?;
        match op._string.as_str() {
            "-" => {
                if let Value::Number(n) = right {
                    Ok(Value::Number(-n))
                } else {
                    Err(RuntimeError::new(
                        String::from("Operand must be an number."),
                        self.line,
                    ))
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

    pub fn EvaluateLiteral(&self, literal: &ExprLiteral) -> Result<Value, RuntimeError> {
        match literal {
            ExprLiteral::Bool(truthy) => Ok(Value::Bool(*truthy)),
            ExprLiteral::String(Stringy) => Ok(Value::String(Stringy.to_owned())),
            ExprLiteral::Number(numy) => {
                let f32 = numy.parse::<f32>().unwrap();
                Ok(Value::Number(f32))
            }
            ExprLiteral::Identifier(str) => {
                let val = self.symbols.get(str);
                if val.is_none() {
                    return Err(RuntimeError::new(
                        format!("Undefined variable '{}'.", str),
                        self.line,
                    ));
                }
                Ok(val.unwrap().clone())
            }
            ExprLiteral::Nil => Ok(Value::Nil),
        }
    }
}
