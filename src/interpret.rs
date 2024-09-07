use crate::{
    eval::{Evaluator, RuntimeError}, expr::Expr, smnt::Statment
};

pub struct Interpreter {
    statements: Vec<Statment>,
    line: i32,
}

impl Interpreter {
    pub fn new(data: Vec<Statment>) -> Self {
        Self {
            statements: data,
            line: 0,
        }
    }

    pub fn interpret(&self) -> Result<(), RuntimeError> {
        for statments in &self.statements {
            match statments {
                Statment::PrintStmt(expr) => {
                    let value = Evaluator::evaluate(expr);
                    match value {
                        Ok(val) => {
                            println!("{val}")
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Statment::ExprStmt(expr) => {
                    let value = Evaluator::evaluate(expr);

                    match value {
                        Ok(val) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
