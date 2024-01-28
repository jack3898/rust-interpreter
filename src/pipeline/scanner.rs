use std::{collections::HashMap, sync::Once};

use crate::{
    error::{CodeLocation, DbgDisplay},
    types::{Lit, Tok, TokType},
    util::string::{is_alpha, is_alphanumeric, is_digit, parse_string},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Tok>,
    /// The start index of the source of the current token. If a string token is being scanned for example, this could be a lot smaller than current.
    start: usize,
    /// The current index of the character in the source code being evaluated. Atomically incremented on a per-character basis.
    current: usize,
    line: usize,
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unexpected EOF.")]
    UnexpectedEof,
    #[error("Unterminated string. All strings must close. {0}.")]
    UnterminatedString(CodeLocation),
    #[error("Could not convert a string into a number. {0}.")]
    InvalidNumber(CodeLocation),
    #[error("Unexpected token. {0}.")]
    UnknownToken(CodeLocation),
}

static mut KEYWORDS: Option<HashMap<&'static str, TokType>> = None;
static INIT: Once = Once::new();

fn get_keywords() -> &'static HashMap<&'static str, TokType> {
    unsafe {
        INIT.call_once(|| {
            KEYWORDS = Some(HashMap::from([
                ("and", TokType::And),
                ("class", TokType::Class),
                ("else", TokType::Else),
                ("false", TokType::False),
                ("for", TokType::For),
                ("fun", TokType::Fun),
                ("if", TokType::If),
                ("nil", TokType::Nil),
                ("or", TokType::Or),
                ("print", TokType::Print),
                ("return", TokType::Return),
                ("super", TokType::Super),
                ("this", TokType::This),
                ("true", TokType::True),
                ("var", TokType::Var),
                ("while", TokType::While),
            ]));
        });
        KEYWORDS.as_ref().unwrap()
    }
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    /// Initialise the scan token process.
    /// Loops through all chars in the source,
    /// updates `self.start` and `self.current` and finishes when at the final character
    /// To calculate what tokens are present
    pub fn scan_tokens(&mut self) -> Result<&Vec<Tok>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Tok {
            token_type: TokType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<TokType, ScannerError> {
        let character = self.advance().ok_or(ScannerError::UnexpectedEof)?;

        // Single character lexemes
        match character {
            '(' => Ok(self.add_token(TokType::LeftParen, None)),
            ')' => Ok(self.add_token(TokType::RightParen, None)),
            '{' => Ok(self.add_token(TokType::LeftBrace, None)),
            '}' => Ok(self.add_token(TokType::RightBrace, None)),
            ',' => Ok(self.add_token(TokType::Comma, None)),
            '.' => Ok(self.add_token(TokType::Dot, None)),
            '-' => Ok(self.add_token(TokType::Minus, None)),
            '+' => Ok(self.add_token(TokType::Plus, None)),
            ';' => Ok(self.add_token(TokType::Semicolon, None)),
            '*' => Ok(self.add_token(TokType::Star, None)),
            '"' => Ok(self.scan_string()?),
            '/' => {
                if self.consume('/') {
                    Ok(self.skip_comment())
                } else if self.consume('*') {
                    Ok(self.skip_block_comment())
                } else {
                    Ok(self.add_token(TokType::Slash, None))
                }
            }
            '!' => {
                let token_type = if self.consume('=') {
                    TokType::BangEqual
                } else {
                    TokType::Equal
                };

                Ok(self.add_token(token_type, None))
            }
            '=' => {
                let token_type = if self.consume('=') {
                    TokType::EqualEqual
                } else {
                    TokType::Equal
                };

                Ok(self.add_token(token_type, None))
            }
            '<' => {
                let token_type = if self.consume('=') {
                    TokType::LessEqual
                } else {
                    TokType::Less
                };

                Ok(self.add_token(token_type, None))
            }
            '>' => {
                let token_type = if self.consume('=') {
                    TokType::GreaterEqual
                } else {
                    TokType::Greater
                };

                Ok(self.add_token(token_type, None))
            }
            '\r' | ' ' => Ok(TokType::None),
            '\n' => {
                self.line += 1;

                Ok(TokType::None)
            }
            any_char => {
                if is_digit(any_char) {
                    Ok(self.scan_number()?)
                } else if is_alpha(character) {
                    Ok(self.scan_ident()?)
                } else {
                    Err(ScannerError::UnknownToken(CodeLocation {
                        line: self.line,
                        display: DbgDisplay::from(&any_char),
                    }))
                }
            }
        }
    }

    fn skip_comment(&mut self) -> TokType {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }

        self.line += 1;

        TokType::None
    }

    fn skip_block_comment(&mut self) -> TokType {
        while self.peek() != '*' && self.peek_at(self.current + 1) != '/' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        self.advance();
        self.advance();

        TokType::None
    }

    /// This method is usually called when a quotation mark has been identified int the source code.
    /// This method will collate all characters in the source code up until the point a closing quotation mark is found.
    /// Will return that collection of characters as a string token type.
    fn scan_string(&mut self) -> Result<TokType, ScannerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString(CodeLocation {
                line: self.line,
                display: DbgDisplay::from(self.tokens.last().unwrap()),
            }));
        }

        // Consume the last " char
        self.advance();

        let value = self.get_source_slice(self.start + 1, self.current - 1); // +1 and -1 to exclude the quotation marks.
        let added_token = self.add_token(TokType::String, Some(Lit::String(value)));

        Ok(added_token)
    }

    fn scan_number(&mut self) -> Result<TokType, ScannerError> {
        while is_digit(self.peek()) {
            self.advance();
        }

        let peek_next = self.peek_at(self.current + 1);

        if self.peek() == '.' && is_digit(peek_next) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.get_source_slice(self.start, self.current);

        let number = parse_string(&value).ok_or(ScannerError::InvalidNumber(CodeLocation {
            line: self.line,
            display: DbgDisplay::from(&value),
        }))?;

        let added_token = self.add_token(TokType::Number, Some(Lit::Number(number)));

        Ok(added_token)
    }

    fn scan_ident(&mut self) -> Result<TokType, ScannerError> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let value = self.get_source_slice(self.start, self.current);

        let token_type_2 = get_keywords().get(value.as_str()).cloned();

        if let Some(token_type) = token_type_2 {
            return Ok(self.add_token(token_type, None));
        }

        Ok(self.add_token(TokType::Identifier, None))
    }

    /// Returns a boolean indicating whether the character was successfully consumed.
    /// Advances current if should a character be consumed.
    fn consume(&mut self, character: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_char_opt = self.source.get(self.current);

        if let Some(current_char) = current_char_opt {
            if current_char != &character {
                return false;
            }
        }

        self.current += 1;

        true
    }

    // Return a bool to identify if the scanner's current position is at the end of the file.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap() // usize always converts to u64, unwrap here is safe.
    }

    /// Return at the char found at self.current. Does not advance self.current. \0 if is at EOF.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.peek_at(self.current)
    }

    /// Peek at a specific index
    fn peek_at(&self, index: usize) -> char {
        self.source.get(index).copied().unwrap_or('\0')
    }

    /// Peek & advance the current index by 1.
    /// Returns the just consumed character, or None if at EOF.
    fn advance(&mut self) -> Option<char> {
        let character = self.peek();

        if character == '\0' {
            return None;
        }

        self.current += 1;

        Some(character)
    }

    /// The slice in the scanner is stored as a vector of chars. This will concatenate a slice and return a string.
    fn get_source_slice<'a>(&self, start: usize, end: usize) -> String {
        self.source
            .get(start..end)
            .expect("Critical error in scanning source code. Attempted to extract a slice of source with an out of bounds index.")
            .iter()
            .collect()
    }

    fn add_token(&mut self, token_type: TokType, literal_type: Option<Lit>) -> TokType {
        let start = self.start;
        let current = self.current;
        let lexeme = self.get_source_slice(start, current);

        let token = Tok {
            token_type: token_type.clone(),
            lexeme,
            literal: literal_type,
            line: self.line,
        };

        self.tokens.push(token);

        token_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{CRLF, LF},
        pipeline::scanner::ScannerError,
        types::{Lit, Tok},
    };

    use super::{Scanner, TokType};

    #[test]
    fn should_scan_with_token_combo() {
        let source = r#"
        (( )) {{ }} , . - +
        ; * / ! = < > 1 10
        200 3000 ident ident2
        "#;

        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 26);
        assert_eq!(scanner.line, 5);
    }

    #[test]
    fn should_ignore_comments() {
        let source_with_comments = r#"
        (( )) {{ }} , . - +
        ; * / ! = < > 1 10
        200 3000 ident ident2
        // This comment should be ignored
        /*
            This multiline comment should be ignored.
        */
        "#;

        let source_without_comments = r#"
        (( )) {{ }} , . - +
        ; * / ! = < > 1 10
        200 3000 ident ident2
        "#;

        let mut scanner_comments = Scanner::new(source_with_comments);
        let mut scanner_no_comments = Scanner::new(source_without_comments);

        scanner_comments.scan_tokens().ok();
        scanner_no_comments.scan_tokens().ok();

        assert_eq!(
            scanner_comments.tokens.len(),
            scanner_no_comments.tokens.len()
        )
    }

    #[test]
    fn should_increment_line_on_crlf() {
        let source = CRLF.repeat(3);
        let mut scanner = Scanner::new(source.as_ref());

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_increment_line_on_lf() {
        let source = LF.repeat(3);
        let mut scanner = Scanner::new(source.as_ref());

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_match_combo_tokens() {
        let source = "!= <= 60 >= == 123.45";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn should_match_string_literal() {
        let source = "\"hey\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(
            scanner.tokens[0].literal,
            Some(Lit::String("hey".to_string()))
        );
    }

    #[test]
    fn should_match_string_literal_with_other_chars() {
        let source = "\"hey, time to be happy! :)\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(
            scanner.tokens[0].literal,
            Some(Lit::String("hey, time to be happy! :)".to_string()))
        );
    }

    #[test]
    fn should_match_string_literal_wrapped_by_chars() {
        let source = "(\"hey, time to be happy! q:)|=<; \")";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(
            scanner.tokens[1].literal,
            Some(Lit::String("hey, time to be happy! q:)|=<; ".to_string())),
        );
    }

    #[test]
    fn should_match_number() {
        let source = "100";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].literal, Some(Lit::Number(100.0)));
    }

    #[test]
    fn should_match_float_number() {
        let source = "10.1;";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].literal, Some(Lit::Number(10.1)));
    }

    #[test]
    fn should_match_identifier() {
        let source = "rando identifier";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].token_type, TokType::Identifier);
        assert_eq!(scanner.tokens[1].token_type, TokType::Identifier);
    }

    #[test]
    fn should_match_embedded_identifier() {
        let source = "rando = identifier + 1";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].token_type, TokType::Identifier);
        assert_eq!(scanner.tokens[2].token_type, TokType::Identifier);
    }

    #[test]
    fn should_match_token_type_from_hash() {
        let source = "var wow = 1 + 1";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].token_type, TokType::Var);
        assert_eq!(scanner.tokens[1].token_type, TokType::Identifier);
    }

    #[test]
    fn should_have_one_eof_token_on_empty_input() {
        let source = "";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            scanner.tokens[0],
            Tok {
                lexeme: "".into(),
                line: 1,
                literal: None,
                token_type: TokType::Eof
            }
        )
    }

    #[test]
    fn should_have_one_eof_token_on_whitespace_input() {
        let source = " ";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            scanner.tokens[0],
            Tok {
                lexeme: "".into(),
                line: 1,
                literal: None,
                token_type: TokType::Eof
            }
        )
    }

    #[test]
    fn should_handle_invalid_tokens() {
        let source = "£";
        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(matches!(result, Err(ScannerError::UnknownToken(_))));
    }
}
