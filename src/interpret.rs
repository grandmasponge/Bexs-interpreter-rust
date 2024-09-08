use crate::{
    eval::{Evaluator, RuntimeError},
    expr::{Expr, ExprLiteral},
    smnt::Statment,
};

pub struct Interpreter {
    statements: Vec<Statment>,
    evaluater: Evaluator,
    line: i32,
}

impl Interpreter {
    pub fn new(data: Vec<Statment>) -> Self {
        Self {
            statements: data,
            evaluater: Evaluator::new(),
            line: 0,
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        for statments in &self.statements {
            match statments {
                Statment::PrintStmt(expr) => {
                    let value = self.evaluater.evaluate(expr);
                    match value {
                        Ok(val) => {
                            println!("{val}")
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Statment::VarDec(Name, value) => {
                    let variable_name = if let Expr::Literal(ExprLiteral::Identifier(str)) = Name {
                        str.clone().to_string()
                    } else {
                        return Err(RuntimeError::new(
                            "failed to interpret variable name".to_string(),
                            self.evaluater.line,
                        ));
                    };
                    let val = self.evaluater.evaluate(value)?;
                    self.evaluater.symbols.insert(variable_name, val);
                }
                Statment::ExprStmt(expr) => {
                    let value = self.evaluater.evaluate(expr);

                    match value {
                        Ok(val) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            self.evaluater.line += 1;
        }
        Ok(())
    }
}
