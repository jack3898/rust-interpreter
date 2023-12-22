use crate::{literal_type::Lit, token::Token, token_type::Tok};

use super::ast::Expr;

// I know there is so much repetition in this file and unoptimised code 🤣 but it'll do for my first prototype

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .expect("Cannot pull token type from current index.")
            .token_type
            == Tok::Eof
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn advance_if_token(&mut self, token_type: &Tok) {
        let token = self.peek().expect("Could not consume current token.");

        if token_type == &token.token_type {
            self.advance();
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn match_token(&self, token_type: &Tok) -> bool {
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
    fn match_tokens_then_advance(&mut self, token_types: &Vec<Tok>) -> bool {
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

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        let token_types = [Tok::BangEqual, Tok::EqualEqual].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in equality.")
                .clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        let token_types = [Tok::Greater, Tok::GreaterEqual, Tok::Less, Tok::LessEqual].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in comparison.")
                .clone();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        let token_types = [Tok::Minus, Tok::Plus, Tok::Star, Tok::Slash].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        let token_types = [Tok::Minus, Tok::Plus].to_vec();

        while self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        let token_types = [Tok::Bang, Tok::Minus].to_vec();

        if self.match_tokens_then_advance(&token_types) {
            let operator = self
                .previous()
                .expect("Could not find previous token in term.")
                .clone();
            let right = self.unary()?;

            let unary = Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            };

            return Ok(unary);
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens_then_advance(&[Tok::True].to_vec()) {
            return Ok(Expr::Literal {
                value: Lit::Bool(true),
            });
        }

        if self.match_tokens_then_advance(&[Tok::False].to_vec()) {
            return Ok(Expr::Literal {
                value: Lit::Bool(false),
            });
        }

        if self.match_tokens_then_advance(&[Tok::Nil].to_vec()) {
            return Ok(Expr::Literal { value: Lit::Nil });
        }

        if self.match_tokens_then_advance(&[Tok::Number, Tok::String].to_vec()) {
            return Ok(Expr::Literal {
                value: self
                    .previous()
                    .ok_or("Could you get previous value in primary.".to_string())?
                    .clone()
                    .literal
                    .ok_or("Access to literal type failed.".to_string())?,
            });
        }

        if self.match_tokens_then_advance(&[Tok::LeftParen].to_vec()) {
            let expr = self.expression()?;

            self.advance_if_token(&Tok::RightParen);

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err("Expecting an expression but did not get one.".to_string())
    }

    #[allow(dead_code)]
    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().unwrap().token_type == Tok::Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                Tok::Class
                | Tok::Fun
                | Tok::Var
                | Tok::For
                | Tok::If
                | Tok::While
                | Tok::Print
                | Tok::Return => return,
                _ => {
                    self.advance().unwrap();
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::scanner::scanner::Scanner;
    use crate::{literal_type::Lit, token::Token, token_type::Tok};

    #[test]
    fn should_add() {
        let one = Token {
            lexeme: "1".to_string(),
            line: 1,
            literal: Some(Lit::Number(1.0)),
            token_type: Tok::Number,
        };

        let plus = Token {
            lexeme: "+".to_string(),
            line: 1,
            literal: None,
            token_type: Tok::Plus,
        };

        let two = Token {
            lexeme: "2".to_string(),
            line: 1,
            literal: Some(Lit::Number(2.0)),
            token_type: Tok::Number,
        };

        let semi = Token {
            lexeme: ';'.to_string(),
            line: 1,
            literal: None,
            token_type: Tok::Semicolon,
        };

        let scanned_tokens = vec![one, plus, two, semi];

        let mut parser = Parser::new(&scanned_tokens);
        let expr = parser.parse();

        assert_eq!(expr.unwrap().to_string(), "(+ 1 2)");
    }

    #[test]
    fn input_from_scanner() {
        let source = "1 + 2 <= 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse();

        assert_eq!(expr.unwrap().to_string(), "(<= (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn input_from_scanner_with_parens() {
        let source = "1 + (2 + 2) == 5";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse();

        assert_eq!(expr.unwrap().to_string(), "(== (+ 1 (group (+ 2 2))) 5)");
    }
}
