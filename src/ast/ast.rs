use crate::{literal_type::Lit, token::Token};

pub enum Expr {
    // two-operands (the items on either side of the operator) like 1 + 1 or 3 != 2
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Lit,
    },
    // something like !x or x++
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Self::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                (*left).to_string(),
                (*right).to_string()
            ),
            Self::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Self::Literal { value } => value.to_string(),
            Self::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, (*right).to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use crate::literal_type::Lit;
    use crate::{token::Token, token_type::Tok};

    #[test]
    fn should_pretty_print_plus() {
        let ast = Expr::Binary {
            left: Box::from(Expr::Literal {
                value: Lit::Number(5.0),
            }),
            operator: Token {
                lexeme: "+".to_string(),
                line: 1,
                literal: None,
                token_type: Tok::Plus,
            },
            right: Box::from(Expr::Literal {
                value: Lit::Number(3.0),
            }),
        };

        assert_eq!("(+ 5 3)", ast.to_string())
    }

    #[test]
    fn should_pretty_print_deep() {
        let ast = Expr::Binary {
            left: Box::from(Expr::Grouping {
                expression: Box::from(Expr::Unary {
                    operator: Token {
                        lexeme: "/".to_string(),
                        line: 1,
                        literal: None,
                        token_type: Tok::Slash,
                    },
                    right: Box::from(Expr::Literal {
                        value: Lit::Number(1.0),
                    }),
                }),
            }),
            operator: Token {
                lexeme: "+".to_string(),
                line: 1,
                literal: None,
                token_type: Tok::Plus,
            },
            right: Box::from(Expr::Literal {
                value: Lit::Number(3.0),
            }),
        };

        assert_eq!("(+ (group (/ 1)) 3)", ast.to_string())
    }
}
