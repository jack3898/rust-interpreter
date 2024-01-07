// Yeah, this could be broken up. But I'm lazy and it works, and I will refactor it later.

use crate::types::{expr::Expr, literal_type::Lit, token_type::TokType};

use super::environment::Environment;

pub struct Evaluator<'a> {
    expression: &'a Expr,
}

impl<'a> Evaluator<'a> {
    pub fn new(expression: &'a Expr) -> Self {
        Self { expression }
    }

    pub fn evaluate(
        &self,
        expr: Option<&'a Expr>,
        environment: &mut Environment,
    ) -> Result<Lit, String> {
        let current_expr = expr.unwrap_or(&self.expression);

        let value: Lit = match current_expr {
            Expr::Variable { name } => match (*environment).get(&name.lexeme) {
                Some(value) => (*value).clone(),
                None => panic!("A variable has been used that has not been defined."),
            },
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expression } => self.evaluate(Some(expression), environment)?.clone(),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(Some(&right), environment)?;

                if let Lit::Number(num) = right {
                    return match operator.token_type {
                        TokType::Minus => Ok(Lit::Number(-(num))),
                        TokType::Plus => Ok(Lit::Number(num)),
                        TokType::Bang => Ok(Lit::Bool(num == 0.0)),
                        _ => Err(
                            "Unexpected token type when evaluating unary for number evaluation."
                                .to_string(),
                        ),
                    };
                }

                if let Lit::Bool(boolean) = right {
                    return match operator.token_type {
                        TokType::Bang => Ok(Lit::Bool(!boolean)),
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

                return Err("Unexpected and/or unidentifiable literal type.".to_string());
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let binary_op = (
                    self.evaluate(Some(&left), environment)?,
                    operator,
                    self.evaluate(Some(&right), environment)?,
                );

                if operator.token_type == TokType::EqualEqual {
                    return Ok(Lit::Bool(binary_op.0 == binary_op.2));
                }

                if operator.token_type == TokType::BangEqual {
                    return Ok(Lit::Bool(binary_op.0 != binary_op.2));
                }

                return match binary_op {
                    (Lit::Number(l), operator, Lit::Number(r)) => match operator.token_type {
                        TokType::Plus => Ok(Lit::Number(l + r)),
                        TokType::Minus => Ok(Lit::Number(l - r)),
                        TokType::Star => Ok(Lit::Number(l * r)),
                        TokType::Slash => Ok(Lit::Number(l / r)),
                        TokType::Greater => Ok(Lit::Bool(l > r)),
                        TokType::GreaterEqual => Ok(Lit::Bool(l >= r)),
                        TokType::Less => Ok(Lit::Bool(l < r)),
                        TokType::LessEqual => Ok(Lit::Bool(l <= r)),
                        _ => Err(
                            "Unexpected token type when evaluating binary for number evaluation."
                                .to_string(),
                        ),
                    },
                    (Lit::String(l), operator, Lit::String(r)) => match operator.token_type {
                        TokType::Plus => Ok(Lit::String(format!("{}{}", l, r))),
                        _ => Err(
                            "Unexpected token type when evaluating binary for string evaluation."
                                .to_string(),
                        ),
                    },
                    (Lit::Bool(_), operator, Lit::Bool(_)) => match operator.token_type {
                        _ => Err(
                            "Unexpected token type when evaluating binary for boolean evaluation."
                                .to_string(),
                        ),
                    },
                    (Lit::Nil, operator, Lit::Nil) => match operator.token_type {
                        _ => Err(
                            "Unexpected token type when evaluating binary for nil evaluation."
                                .to_string(),
                        ),
                    },
                    _ => Err("Unexpected and unidentifiable literal type.".to_string()),
                };
            }
        };

        Ok(value)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         pipeline::evaluator::Evaluator,
//         types::{expr::Expr, literal_type::Lit, token::Tok, token_type::TokType},
//     };

//     fn test_create_expr_literal(literal: Lit) -> Box<Expr> {
//         Box::new(Expr::Literal { value: literal })
//     }

//     fn test_create_operator(operator: TokType, lexeme: &str) -> Tok {
//         Tok {
//             token_type: operator,
//             lexeme: lexeme.to_string(),
//             literal: None,
//             line: 0,
//         }
//     }

//     // UNARY
//     #[test]
//     fn should_evaluate_unary_bang() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Bang, "!"),
//             right: test_create_expr_literal(Lit::Bool(true)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(false));
//     }

//     #[test]
//     fn should_evaluate_unary_bang_with_0_number() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Bang, "!"),
//             right: test_create_expr_literal(Lit::Number(0.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(result, Ok(Lit::Bool(true)))
//     }

//     #[test]
//     fn should_evaluate_unary_bang_with_other_number() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Bang, "!"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(result, Ok(Lit::Bool(false)))
//     }

//     #[test]
//     fn should_evaluate_unary_minus() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Minus, "-"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(-1.0));
//     }

//     #[test]
//     fn should_evaluate_unary_plus() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(1.0));
//     }

//     #[test]
//     fn should_evaluate_unary_plus_with_0_number() {
//         let expression = Expr::Unary {
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Number(0.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(0.0));
//     }

//     // BINARY

//     #[test]
//     fn should_sum_numbers() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(3.0));
//     }

//     #[test]
//     fn should_subtract_numbers() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::Minus, "-"),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(-1.0));
//     }

//     #[test]
//     fn should_multiply_numbers() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(2.0)),
//             operator: test_create_operator(TokType::Star, "*"),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(4.0));
//     }

//     #[test]
//     fn should_divide_numbers() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(2.0)),
//             operator: test_create_operator(TokType::Slash, "/"),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(1.0));
//     }

//     #[test]
//     fn should_evaluate_greater_than() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(2.0)),
//             operator: test_create_operator(TokType::Greater, ">"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_greater_than_equal() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(2.0)),
//             operator: test_create_operator(TokType::GreaterEqual, ">="),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_less_than() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::Less, "<"),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_less_than_equal() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::LessEqual, "<="),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_equal_equal() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::EqualEqual, "=="),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_bang_equal() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::BangEqual, "!="),
//             right: test_create_expr_literal(Lit::Number(2.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Bool(true));
//     }

//     #[test]
//     fn should_evaluate_bang_equal_with_different_types() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::BangEqual, "!="),
//             right: test_create_expr_literal(Lit::Bool(true)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(result, Ok(Lit::Bool(true)));
//     }

//     #[test]
//     fn should_evaluate_bang_equal_with_same_types() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::BangEqual, "!="),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(result, Ok(Lit::Bool(false)));
//     }

//     #[test]
//     fn should_evaluate_plus_with_strings() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::String("Hello".to_string())),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::String("World".to_string())),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(result, Ok(Lit::String("HelloWorld".to_string())));
//     }

//     #[test]
//     fn should_evaluate_plus_with_string_and_number() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::String("Hello".to_string())),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_number_and_string() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::String("Hello".to_string())),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_number_and_bool() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Number(1.0)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Bool(true)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_bool_and_number() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Bool(true)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_bool_and_bool() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Bool(true)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Bool(true)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_string_and_bool() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::String("Hello".to_string())),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Bool(true)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_bool_and_string() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Bool(true)),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::String("Hello".to_string())),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_string_and_nil() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::String("Hello".to_string())),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::Nil),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     #[test]
//     fn should_evaluate_plus_with_nil_and_string() {
//         let expression = Expr::Binary {
//             left: test_create_expr_literal(Lit::Nil),
//             operator: test_create_operator(TokType::Plus, "+"),
//             right: test_create_expr_literal(Lit::String("Hello".to_string())),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None);

//         assert_eq!(matches!(result, Err(_)), true);
//     }

//     // GROUPING
//     #[test]
//     fn should_evaluate_grouping() {
//         let expression = Expr::Grouping {
//             expression: test_create_expr_literal(Lit::Number(1.0)),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(1.0));
//     }

//     #[test]
//     fn should_evaluate_grouping_with_binary() {
//         let expression = Expr::Grouping {
//             expression: Box::new(Expr::Binary {
//                 left: test_create_expr_literal(Lit::Number(1.0)),
//                 operator: test_create_operator(TokType::Plus, "+"),
//                 right: test_create_expr_literal(Lit::Number(2.0)),
//             }),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(3.0));
//     }

//     #[test]
//     fn should_evaluate_grouping_with_unary() {
//         let expression = Expr::Grouping {
//             expression: Box::new(Expr::Unary {
//                 operator: test_create_operator(TokType::Minus, "-"),
//                 right: test_create_expr_literal(Lit::Number(1.0)),
//             }),
//         };

//         let interpreter = Evaluator::new(&expression);
//         let result = interpreter.evaluate(None).unwrap();

//         assert_eq!(result, Lit::Number(-1.0));
//     }
// }
