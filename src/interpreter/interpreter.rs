use crate::{ast::ast::Expr, literal_type::Lit, token_type::Tok};

// Yeah, this could be broken up. But I'm lazy and it works, and I will refactor it later.

pub struct Interpreter<'a> {
    expression: &'a Expr,
}

impl<'a> Interpreter<'a> {
    pub fn new(expression: &'a Expr) -> Self {
        Self { expression }
    }

    pub fn evaluate(&self, expr: Option<&'a Expr>) -> Result<Lit, String> {
        let current_expr = expr.unwrap_or(&self.expression);

        let value: Lit = match current_expr {
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate(Some(expression))?.clone(),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(Some(&right))?;

                if let Lit::Number(num) = right {
                    return match operator.token_type {
                        Tok::Minus => Ok(Lit::Number(-(num))),
                        Tok::Plus => Ok(Lit::Number(num)),
                        Tok::Bang => Ok(Lit::Bool(num == 0.0)),
                        _ => Err(
                            "Unexpected token type when evaluating unary for number evaluation."
                                .to_string(),
                        ),
                    };
                }

                if let Lit::Bool(boolean) = right {
                    return match operator.token_type {
                        Tok::Bang => Ok(Lit::Bool(!boolean)),
                        _ => Err(
                            "Unexpected token type when evaluating unary for boolean evaluation."
                                .to_string(),
                        ),
                    };
                }

                if let Lit::Nil = right {
                    return Err("Illegal use of nil in unary.".to_string());
                }

                if let Lit::String(_) = right {
                    return Err("Illegal use of string in unary.".to_string());
                }

                return Err("Unexpected and unidentifiable literal type.".to_string());
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let literals = (self.evaluate(Some(&left))?, self.evaluate(Some(&right))?);

                match operator.token_type {
                    Tok::EqualEqual => {
                        return Ok(Lit::Bool(literals.0 == literals.1));
                    }
                    Tok::BangEqual => {
                        return Ok(Lit::Bool(literals.0 != literals.1));
                    }
                    Tok::Plus => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Number(l + r)),
                        (Lit::String(l), Lit::String(r)) => {
                            return Ok(Lit::String(format!("{}{}", l, r)))
                        }
                        _ => {
                            return Err(
                                "Attempt to add two unmatching or illegal data types.".to_string()
                            )
                        }
                    },
                    Tok::Minus => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Number(l - r)),
                        _ => {
                            return Err("Attempt to subtract two unmatching or illegal data types."
                                .to_string())
                        }
                    },
                    Tok::Slash => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Number(l / r)),
                        _ => {
                            return Err("Attempt to divide two unmatching or illegal data types."
                                .to_string())
                        }
                    },
                    Tok::Star => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Number(l * r)),
                        _ => {
                            return Err("Attempt to multiply two unmatching or illegal data types."
                                .to_string())
                        }
                    },
                    Tok::Greater => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Bool(l > r)),
                        (Lit::String(l), Lit::String(r)) => {
                            return Ok(Lit::Bool(l.len() > r.len()));
                        }
                        _ => return Err("Invalid use of greater operator.".to_string()),
                    },
                    Tok::GreaterEqual => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Bool(l >= r)),
                        (Lit::String(l), Lit::String(r)) => {
                            return Ok(Lit::Bool(l.len() >= r.len()));
                        }
                        _ => return Err("Invalid use of greater-equal operator.".to_string()),
                    },
                    Tok::Less => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Bool(l < r)),
                        (Lit::String(l), Lit::String(r)) => {
                            return Ok(Lit::Bool(l.len() < r.len()));
                        }
                        _ => return Err("Invalid use of less operator.".to_string()),
                    },
                    Tok::LessEqual => match literals {
                        (Lit::Number(l), Lit::Number(r)) => return Ok(Lit::Bool(l <= r)),
                        (Lit::String(l), Lit::String(r)) => {
                            return Ok(Lit::Bool(l.len() <= r.len()));
                        }
                        _ => return Err("Invalid use of greater-equal operator.".to_string()),
                    },
                    _ => return Err("Unable to evaluate binary expression.".to_string()),
                }
            }
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ast::Expr, interpreter::interpreter::Interpreter, literal_type::Lit, token::Token,
        token_type::Tok,
    };

    fn test_create_expr_literal(literal: Lit) -> Box<Expr> {
        Box::new(Expr::Literal { value: literal })
    }

    fn test_create_operator(operator: Tok, lexeme: &str) -> Token {
        Token {
            token_type: operator,
            lexeme: lexeme.to_string(),
            literal: None,
            line: 0,
        }
    }

    // UNARY
    #[test]
    fn should_evaluate_unary_bang() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Bang, "!"),
            right: test_create_expr_literal(Lit::Bool(true)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(false));
    }

    #[test]
    fn should_evaluate_unary_bang_with_0_number() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Bang, "!"),
            right: test_create_expr_literal(Lit::Number(0.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(result, Ok(Lit::Bool(true)))
    }

    #[test]
    fn should_evaluate_unary_bang_with_other_number() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Bang, "!"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(result, Ok(Lit::Bool(false)))
    }

    #[test]
    fn should_evaluate_unary_minus() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Minus, "-"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(-1.0));
    }

    #[test]
    fn should_evaluate_unary_plus() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(1.0));
    }

    #[test]
    fn should_evaluate_unary_plus_with_0_number() {
        let expression = Expr::Unary {
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Number(0.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(0.0));
    }

    // BINARY

    #[test]
    fn should_sum_numbers() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(3.0));
    }

    #[test]
    fn should_subtract_numbers() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::Minus, "-"),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(-1.0));
    }

    #[test]
    fn should_multiply_numbers() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(2.0)),
            operator: test_create_operator(Tok::Star, "*"),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(4.0));
    }

    #[test]
    fn should_divide_numbers() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(2.0)),
            operator: test_create_operator(Tok::Slash, "/"),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(1.0));
    }

    #[test]
    fn should_evaluate_greater_than() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(2.0)),
            operator: test_create_operator(Tok::Greater, ">"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_greater_than_equal() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(2.0)),
            operator: test_create_operator(Tok::GreaterEqual, ">="),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_less_than() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::Less, "<"),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_less_than_equal() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::LessEqual, "<="),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_equal_equal() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::EqualEqual, "=="),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_bang_equal() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::BangEqual, "!="),
            right: test_create_expr_literal(Lit::Number(2.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Bool(true));
    }

    #[test]
    fn should_evaluate_bang_equal_with_different_types() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::BangEqual, "!="),
            right: test_create_expr_literal(Lit::Bool(true)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(result, Ok(Lit::Bool(true)));
    }

    #[test]
    fn should_evaluate_bang_equal_with_same_types() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::BangEqual, "!="),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(result, Ok(Lit::Bool(false)));
    }

    #[test]
    fn should_evaluate_plus_with_strings() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::String("Hello".to_string())),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::String("World".to_string())),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(result, Ok(Lit::String("HelloWorld".to_string())));
    }

    #[test]
    fn should_evaluate_plus_with_string_and_number() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::String("Hello".to_string())),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_number_and_string() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::String("Hello".to_string())),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_number_and_bool() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Number(1.0)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Bool(true)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_bool_and_number() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Bool(true)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_bool_and_bool() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Bool(true)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Bool(true)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_string_and_bool() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::String("Hello".to_string())),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Bool(true)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_bool_and_string() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Bool(true)),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::String("Hello".to_string())),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_string_and_nil() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::String("Hello".to_string())),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::Nil),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    #[test]
    fn should_evaluate_plus_with_nil_and_string() {
        let expression = Expr::Binary {
            left: test_create_expr_literal(Lit::Nil),
            operator: test_create_operator(Tok::Plus, "+"),
            right: test_create_expr_literal(Lit::String("Hello".to_string())),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None);

        assert_eq!(
            result,
            Err("Attempt to add two unmatching or illegal data types.".to_string())
        );
    }

    // GROUPING
    #[test]
    fn should_evaluate_grouping() {
        let expression = Expr::Grouping {
            expression: test_create_expr_literal(Lit::Number(1.0)),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(1.0));
    }

    #[test]
    fn should_evaluate_grouping_with_binary() {
        let expression = Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: test_create_expr_literal(Lit::Number(1.0)),
                operator: test_create_operator(Tok::Plus, "+"),
                right: test_create_expr_literal(Lit::Number(2.0)),
            }),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(3.0));
    }

    #[test]
    fn should_evaluate_grouping_with_unary() {
        let expression = Expr::Grouping {
            expression: Box::new(Expr::Unary {
                operator: test_create_operator(Tok::Minus, "-"),
                right: test_create_expr_literal(Lit::Number(1.0)),
            }),
        };

        let interpreter = Interpreter::new(&expression);
        let result = interpreter.evaluate(None).unwrap();

        assert_eq!(result, Lit::Number(-1.0));
    }
}
