use crate::types::literal_type::Lit;

use super::{environment::Environment, evaluator::Evaluator, stmt::Stmt};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    #[allow(dead_code)]
    pub fn interpret_expr(&mut self, evaluator: &mut Evaluator) -> Result<Lit, String> {
        evaluator.evaluate(None, &mut self.environment)
    }

    pub fn interpret_stmts(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Stmt::Expr { expr } => {
                    let evaluator = Evaluator::new(&expr);

                    evaluator.evaluate(None, &mut self.environment)?;
                }
                Stmt::Print { expr } => {
                    let evaluator = Evaluator::new(&expr);
                    let value = evaluator.evaluate(None, &mut self.environment)?;

                    println!("{value}");
                }
                Stmt::Var { name: token, expr } => {
                    let evaluator = Evaluator::new(&expr);
                    let value = evaluator.evaluate(Some(&expr), &mut self.environment)?;

                    self.environment.define(&token.lexeme, value);
                }
            }
        }

        Ok(())
    }
}
