use crate::types::{expr::Expr, literal_type::Lit, token::Tok, token_type::TokType};

use super::stmt::Stmt;

// I know there is so much repetition in this file and unoptimised code 🤣 but it'll do for my first prototype

pub struct Parser<'a> {
    tokens: &'a Vec<Tok>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Tok>) -> Self {
        Self { current: 0, tokens }
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .expect("Cannot pull token type from current index.")
            .token_type
            == TokType::Eof
    }

    fn advance(&mut self) -> Option<&Tok> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, token_type: &TokType) -> Result<(), String> {
        let token = self.peek().ok_or("Unable peek token".to_string())?;

        if token_type == &token.token_type {
            self.advance();
        } else {
            return Err("Token type not matched.".to_string());
        }

        Ok(())
    }

    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.current)
    }

    fn match_token(&self, token_type: &TokType) -> bool {
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
    fn match_tokens_then_advance(&mut self, token_types: &Vec<TokType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn previous(&self) -> Option<&Tok> {
        self.tokens.get(self.current - 1)
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts: Vec<Stmt> = vec![];
        let mut errors: Vec<String> = vec![];

        while !self.is_at_end() {
            let statement = self.statement();

            match statement {
                Ok(stmt) => stmts.push(stmt),
                Err(reason) => errors.push(reason),
            }
        }

        if errors.len() > 0 {
            return Err(errors.join("\n"));
        }

        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_tokens_then_advance(&[TokType::Print].to_vec()) {
            return self.print_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;

        self.consume(&TokType::Semicolon)?;

        Ok(Stmt::Print { expr: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;

        self.consume(&TokType::Semicolon)?;

        Ok(Stmt::Expr { expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        let token_types = [TokType::BangEqual, TokType::EqualEqual].to_vec();

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
        let token_types = [
            TokType::Greater,
            TokType::GreaterEqual,
            TokType::Less,
            TokType::LessEqual,
        ]
        .to_vec();

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
        let token_types = [TokType::Minus, TokType::Plus, TokType::Star, TokType::Slash].to_vec();

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
        let token_types = [TokType::Minus, TokType::Plus].to_vec();

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
        let token_types = [TokType::Bang, TokType::Minus].to_vec();

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
        if self.match_tokens_then_advance(&[TokType::True].to_vec()) {
            return Ok(Expr::Literal {
                value: Lit::Bool(true),
            });
        }

        if self.match_tokens_then_advance(&[TokType::False].to_vec()) {
            return Ok(Expr::Literal {
                value: Lit::Bool(false),
            });
        }

        if self.match_tokens_then_advance(&[TokType::Nil].to_vec()) {
            return Ok(Expr::Literal { value: Lit::Nil });
        }

        if self.match_tokens_then_advance(&[TokType::Number, TokType::String].to_vec()) {
            return Ok(Expr::Literal {
                value: self
                    .previous()
                    .ok_or("Could you get previous value in primary.".to_string())?
                    .clone()
                    .literal
                    .ok_or("Access to literal type failed.".to_string())?,
            });
        }

        if self.match_tokens_then_advance(&[TokType::LeftParen].to_vec()) {
            let expr = self.expression()?;

            self.consume(&TokType::RightParen);

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
            if self.previous().unwrap().token_type == TokType::Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                TokType::Class
                | TokType::Fun
                | TokType::Var
                | TokType::For
                | TokType::If
                | TokType::While
                | TokType::Print
                | TokType::Return => return,
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
    use crate::{
        pipeline::scanner::Scanner,
        types::{literal_type::Lit, token::Tok, token_type::TokType},
    };

    use super::Parser;

    #[test]
    fn should_add() {
        let one = Tok {
            lexeme: "1".to_string(),
            line: 1,
            literal: Some(Lit::Number(1.0)),
            token_type: TokType::Number,
        };

        let plus = Tok {
            lexeme: "+".to_string(),
            line: 1,
            literal: None,
            token_type: TokType::Plus,
        };

        let two = Tok {
            lexeme: "2".to_string(),
            line: 1,
            literal: Some(Lit::Number(2.0)),
            token_type: TokType::Number,
        };

        let semi = Tok {
            lexeme: ';'.to_string(),
            line: 1,
            literal: None,
            token_type: TokType::Semicolon,
        };

        let scanned_tokens = vec![one, plus, two, semi];

        let mut parser = Parser::new(&scanned_tokens);
        let expr = parser.expression();

        assert_eq!(expr.unwrap().to_string(), "(+ 1 2)");
    }

    #[test]
    fn input_from_scanner() {
        let source = "1 + 2 <= 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);
        let expr = parser.expression();

        assert_eq!(expr.unwrap().to_string(), "(<= (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn input_from_scanner_with_parens() {
        let source = "1 + (2 + 2) == 5";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);
        let expr = parser.expression();

        assert_eq!(expr.unwrap().to_string(), "(== (+ 1 (group (+ 2 2))) 5)");
    }
}
