use thiserror::Error;

use crate::{
    error::{CodeLocation, DbgDisplay},
    types::{expr::Expr, literal_type::Lit, token::Tok, token_type::TokType},
};

use super::stmt::Stmt;

// I know there is so much repetition in this file and unoptimised code 🤣 but it'll do for my first prototype

pub struct Parser<'a> {
    tokens: &'a Vec<Tok>,
    current: usize,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("An unexpected token was found. {0}. Expected {1}")]
    UnexpectedToken(CodeLocation, TokType),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("No primary: {0}")]
    PrimaryError(CodeLocation),
    #[error("No literal type found: {0}")]
    UndefinedLiteral(CodeLocation),
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Tok>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut stmts: Vec<Stmt> = vec![];
        let mut errors: Vec<ParserError> = vec![];

        while !self.is_at_end() {
            let statement = self.declaration();

            match statement {
                Ok(stmt) => stmts.push(stmt),
                Err(error) => errors.push(error),
            }
        }

        if errors.len() > 0 {
            let error = errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            return Err(ParserError::ParseError(error));
        }

        Ok(stmts)
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokType::Eof
    }

    fn advance(&mut self) -> &Tok {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, token_type: TokType) -> Result<&Tok, ParserError> {
        let token = self.peek();

        if token_type == token.token_type {
            self.advance();

            return Ok(self.previous());
        } else {
            return Err(ParserError::UnexpectedToken(
                CodeLocation {
                    line: token.line,
                    display: DbgDisplay::from(token),
                },
                token_type,
            ));
        }
    }

    fn peek(&self) -> &Tok {
        self.tokens
            .get(self.current)
            .expect("Critical error, Cannot peek token!")
    }

    fn match_token(&self, token_type: TokType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    // I could create a macro for variable length params, but this is cleaner and less confusing
    /// Checks each token in a given array. Advances current up until the point a token is matched from the array.
    fn match_tokens_then_advance(&mut self, token_types: &[TokType]) -> bool {
        for token_type in token_types {
            if self.match_token(*token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn previous(&self) -> &Tok {
        self.tokens
            .get(self.current - 1)
            .expect("Critical error, parser is unable to retrieve previous token.")
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_token(TokType::Var) {
            self.advance();

            return self.var_declaration();
        }

        let statement = self.statement();

        if statement.is_err() {
            self.synchronise();
        }

        statement
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let token = self.consume(TokType::Identifier)?.clone();

        let initialiser = if self.match_token(TokType::Equal) {
            self.advance();

            self.expression()?
        } else {
            Expr::Literal { value: Lit::Nil }
        };

        self.consume(TokType::Semicolon)?;

        Ok(Stmt::Var {
            name: token,
            expr: initialiser,
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        return if self.match_tokens_then_advance(&[TokType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        };
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;

        self.consume(TokType::Semicolon)?;

        Ok(Stmt::Print { expr: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;

        self.consume(TokType::Semicolon)?;

        Ok(Stmt::Expr { expr })
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;
        let token_types = [TokType::BangEqual, TokType::EqualEqual];

        while self.match_tokens_then_advance(&token_types) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        let token_types = [
            TokType::Greater,
            TokType::GreaterEqual,
            TokType::Less,
            TokType::LessEqual,
        ];

        while self.match_tokens_then_advance(&token_types) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        let token_types = [TokType::Minus, TokType::Plus, TokType::Star, TokType::Slash];

        while self.match_tokens_then_advance(&token_types) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        let token_types = [TokType::Minus, TokType::Plus];

        while self.match_tokens_then_advance(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokType::Bang, TokType::Minus];

        if self.match_tokens_then_advance(&token_types) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            let unary = Expr::Unary {
                operator: operator,
                right: Box::new(right),
            };

            return Ok(unary);
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_tokens_then_advance(&[TokType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        if self.match_tokens_then_advance(&[TokType::True]) {
            return Ok(Expr::Literal {
                value: Lit::Bool(true),
            });
        }

        if self.match_tokens_then_advance(&[TokType::False]) {
            return Ok(Expr::Literal {
                value: Lit::Bool(false),
            });
        }

        if self.match_tokens_then_advance(&[TokType::Nil]) {
            return Ok(Expr::Literal { value: Lit::Nil });
        }

        if self.match_tokens_then_advance(&[TokType::Number, TokType::String]) {
            return Ok(Expr::Literal {
                value: self
                    .previous()
                    .literal
                    .clone()
                    .ok_or(ParserError::UndefinedLiteral(CodeLocation {
                        line: self.peek().line,
                        display: DbgDisplay::from(self.peek()),
                    }))?,
            });
        }

        if self.match_tokens_then_advance(&[TokType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(TokType::RightParen)?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParserError::PrimaryError(CodeLocation {
            line: self.peek().line,
            display: DbgDisplay::from(self.peek()),
        }))
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokType::Class
                | TokType::Fun
                | TokType::Var
                | TokType::For
                | TokType::If
                | TokType::While
                | TokType::Print
                | TokType::Return => return,
                _ => {
                    self.advance();
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
