use crate::{eval::Evaluator, smnt::Statment};

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

    pub fn interpret(&self) {
        for statments in &self.statements {
            match statments {
                Statment::PrintStmt(Expr) => {
                    let value = Evaluator::Evaluate(Expr).unwrap();
                    println!("{value}");
                }
                Statment::ExprStmt(Expr) => {}
            }
        }
    }
}
