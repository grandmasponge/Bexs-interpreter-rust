use crate::{
    eval::{Evaluator, RuntimeError},
    smnt::Statment,
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
                Statment::PrintStmt(Expr) => {
                    let value = Evaluator::Evaluate(Expr);
                    match value {
                        Ok(val) => {
                            println!("{val}")
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Statment::ExprStmt(Expr) => {}
            }
        }
        Ok(())
    }
}
