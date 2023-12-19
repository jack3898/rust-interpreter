use crate::{literal_type::LiteralType, token::Token, token_type::TokenType};

use super::ast::Expr;

// I know there is so much repetition in this file and unoptimised code 🤣 but it'll do for my first prototype

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .expect("Cannot pull token type from current index.")
            .token_type
            == TokenType::Eof
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn advance_if_token(&mut self, token_type: &TokenType) {
        let token = self.peek().expect("Could not consume current token.");

        if token_type == &token.token_type {
            self.advance();
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn match_token(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self
            .peek()
            .expect("Cannot pull token type from current index.")
            .token_type
            == *token_type;
    }

    // I could create a macro for variable length params, but this is cleaner and less confusing albeit slower
    fn match_tokens_then_advance(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let token_types = [TokenType::BangEqual, TokenType::EqualEqual].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in equality.")
                .clone();
            let right = self.comparison();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let token_types = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]
        .to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in comparison.")
                .clone();
            let right = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.unary();
        let token_types = [TokenType::Minus, TokenType::Plus].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let token_types = [TokenType::Minus, TokenType::Plus].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let token_types = [TokenType::Bang, TokenType::BangEqual].to_vec();

        if self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.unary();

            return Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_tokens_then_advance(&[TokenType::True].to_vec()) {
            return Some(Expr::Literal {
                value: LiteralType::Boolean(true),
            });
        }

        if self.match_tokens_then_advance(&[TokenType::False].to_vec()) {
            return Some(Expr::Literal {
                value: LiteralType::Boolean(false),
            });
        }

        if self.match_tokens_then_advance(&[TokenType::Nil].to_vec()) {
            return Some(Expr::Literal {
                value: LiteralType::Nil,
            });
        }

        if self.match_tokens_then_advance(&[TokenType::Number, TokenType::String].to_vec()) {
            return Some(Expr::Literal {
                value: self
                    .previous()
                    .expect("Could you get previous value in primary.")
                    .clone()
                    .literal
                    .unwrap(),
            });
        }

        if self.match_tokens_then_advance(&[TokenType::LeftParen].to_vec()) {
            let expr = self.expression();

            self.advance_if_token(&TokenType::RightParen);

            return Some(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        None
    }
}

mod tests {
    #[cfg(test)]
    use crate::scanner::scanner::Scanner;
    #[cfg(test)]
    use crate::{literal_type::LiteralType, token::Token, token_type::TokenType};

    #[cfg(test)]
    use super::Parser;

    #[test]
    fn should_add() {
        let one = Token {
            lexeme: "1".to_string(),
            line: 1,
            literal: Some(LiteralType::Number(1.0)),
            token_type: TokenType::Number,
        };

        let plus = Token {
            lexeme: "+".to_string(),
            line: 1,
            literal: None,
            token_type: TokenType::Plus,
        };

        let two = Token {
            lexeme: "2".to_string(),
            line: 1,
            literal: Some(LiteralType::Number(2.0)),
            token_type: TokenType::Number,
        };

        let semicol = Token {
            lexeme: ';'.to_string(),
            line: 1,
            literal: None,
            token_type: TokenType::Semicolon,
        };

        let scanned_tokens = vec![one, plus, two, semicol];

        let mut parser = Parser::new(scanned_tokens);
        let expr = parser.expression();

        assert_eq!(expr.to_string(), "(+ 1 2)");
    }

    #[test]
    fn input_from_scanner() {
        let source = "1 + 2 <= 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens.clone());
        let expr = parser.expression();

        assert_eq!(expr.to_string(), "(<= (+ 1 2) (+ 5 7))");
    }
}
