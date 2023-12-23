use crate::types::literal_type::Lit;

use super::{evaluator::Evaluator, stmt::Stmt};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expr(&mut self, evaluator: &mut Evaluator) -> Result<Lit, String> {
        evaluator.evaluate(None)
    }

    pub fn interpret_stmts(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Stmt::Expr { expr } => {
                    let evaluator = Evaluator::new(&expr);

                    evaluator.evaluate(None)?;
                }
                Stmt::Print { expr } => {
                    let evaluator = Evaluator::new(&expr);
                    let value = evaluator.evaluate(None)?;

                    println!("{value}");
                }
            }
        }

        Ok(())
    }
}
