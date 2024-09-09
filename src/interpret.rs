use crate::{
    eval::{Evaluator, RuntimeError},
    expr::{Expr, ExprLiteral},
    smnt::Statment,
};

pub struct Interpreter {
    evaluater: Evaluator,
    line: i32,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            evaluater: Evaluator::new(),
            line: 0,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statment>) -> Result<(), RuntimeError> {
        for statments in statements {
            match statments {
                Statment::PrintStmt(expr) => {
                    let value = self.evaluater.evaluate(&expr);
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
                    let val = self.evaluater.evaluate(&value)?;
                    self.evaluater.symbols.insert(variable_name, val);
                }
                Statment::BlockStatment(tehes) => {
                    let mut new_enviroment = Interpreter::new();
                    new_enviroment.interpret(tehes.as_ref().clone())?;
                }
                Statment::ExprStmt(expr) => {
                    let value = self.evaluater.evaluate(&expr);

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
