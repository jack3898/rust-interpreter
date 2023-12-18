use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
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

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

pub enum Literal {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
            Self::Str(str) => str.to_string(),
            Self::Number(num) => num.to_string(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string())
    }
}

mod tests {
    #[cfg(test)]
    use crate::{ast::ast::Literal, token::Token, token_type::TokenType};

    #[cfg(test)]
    use super::Literal::Number;

    #[cfg(test)]
    use super::Expr;

    #[test]
    fn should_pretty_print_plus() {
        let ast = Expr::Binary {
            left: Box::from(Expr::Literal { value: Number(5.0) }),
            operator: Token {
                lexeme: "+".to_string(),
                line: 1,
                literal: None,
                token_type: TokenType::Plus,
            },
            right: Box::from(Expr::Literal { value: Number(3.0) }),
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
                        token_type: TokenType::Slash,
                    },
                    right: Box::from(Expr::Literal {
                        value: Literal::Number(1.0),
                    }),
                }),
            }),
            operator: Token {
                lexeme: "+".to_string(),
                line: 1,
                literal: None,
                token_type: TokenType::Plus,
            },
            right: Box::from(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        };

        assert_eq!("(+ (group (/ 1)) 3)", ast.to_string())
    }
}
