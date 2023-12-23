use std::collections::HashMap;

use crate::{
    types::{literal_type::Lit, token::Tok, token_type::TokType},
    util::string::{is_alpha, is_alphanumeric, is_digit, parse_string},
};

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Tok>,
    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokType> = {
        let mut keywords = HashMap::new();

        keywords.insert("and", TokType::And);
        keywords.insert("class", TokType::Class);
        keywords.insert("else", TokType::Else);
        keywords.insert("false", TokType::False);
        keywords.insert("for", TokType::For);
        keywords.insert("fun", TokType::Fun);
        keywords.insert("if", TokType::If);
        keywords.insert("nil", TokType::Nil);
        keywords.insert("or", TokType::Or);
        keywords.insert("print", TokType::Print);
        keywords.insert("return", TokType::Return);
        keywords.insert("super", TokType::Super);
        keywords.insert("this", TokType::This);
        keywords.insert("true", TokType::True);
        keywords.insert("var", TokType::Var);
        keywords.insert("while", TokType::While);

        keywords
    };
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
    /// statefully updates self.start and self.current and finishes when at the final character
    /// To calculate what tokens are present
    pub fn scan_tokens(&mut self) -> Result<&Vec<Tok>, String> {
        while !Self::is_at_end(self) {
            self.start = self.current;
            Self::scan_token(self);
        }

        self.tokens.push(Tok {
            token_type: TokType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Option<TokType> {
        let character = self.advance()?;

        // Single character lexemes
        match character {
            '(' => self.add_token(TokType::LeftParen, None),
            ')' => self.add_token(TokType::RightParen, None),
            '{' => self.add_token(TokType::LeftBrace, None),
            '}' => self.add_token(TokType::RightBrace, None),
            ',' => self.add_token(TokType::Comma, None),
            '.' => self.add_token(TokType::Dot, None),
            '-' => self.add_token(TokType::Minus, None),
            '+' => self.add_token(TokType::Plus, None),
            ';' => self.add_token(TokType::Semicolon, None),
            '*' => self.add_token(TokType::Star, None),
            '/' => self.add_token(TokType::Slash, None), // TODO: Add comment evaluation into this match
            '"' => match self.scan_to_string_token_then_advance() {
                Ok(string_token) => Some(string_token),
                Err(error) => {
                    println!("{}", error);

                    None
                }
            },
            '!' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokType::BangEqual
                } else {
                    TokType::Equal
                };

                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokType::EqualEqual
                } else {
                    TokType::Equal
                };

                self.add_token(token_type, None)
            }
            '<' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokType::LessEqual
                } else {
                    TokType::Less
                };

                self.add_token(token_type, None)
            }
            '>' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokType::GreaterEqual
                } else {
                    TokType::Greater
                };

                self.add_token(token_type, None)
            }
            '\r' | ' ' => None,
            '\n' => {
                self.line += 1;

                None
            }
            any_char => {
                if is_digit(any_char) {
                    match self.scan_to_number_token_then_advance() {
                        Ok(number_token) => Some(number_token),
                        Err(error) => {
                            println!("{}", error);

                            None
                        }
                    }
                } else if is_alpha(character) {
                    match self.scan_to_identifier_then_advance() {
                        Ok(identifier) => Some(identifier),
                        Err(error) => {
                            println!("{}", error);

                            None
                        }
                    }
                } else {
                    println!(
                        "Error parsing source code at char '{}', line {}",
                        character, self.line
                    );

                    None
                }
            }
        }
    }

    /// Identify a string of random lengths.
    /// Keeps scanning until it finds the closing quote and will
    /// Respond with the token. Uses self.start and advances self.current to the closing quote to return the string.
    fn scan_to_string_token_then_advance(&mut self) -> Result<TokType, String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string.".to_string());
        }

        // Consume the last " char
        self.advance();

        let value = self
            .source_slice(self.start + 1, self.current - 1)
            .ok_or("Trouble calculating string literal slice."); // TODO: Rubbish error 😂

        let added_token = self
            .add_token(TokType::String, Some(Lit::String(value?)))
            .ok_or("Could not add token.".to_string()); // TODO: More rubbish error

        added_token
    }

    fn scan_to_number_token_then_advance(&mut self) -> Result<TokType, String> {
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

        let value = self.source_slice(self.start, self.current).unwrap();

        let number =
            parse_string(&value).ok_or(format!("Unable to parse {:?} into a number", value));

        let added_token = self
            .add_token(TokType::Number, Some(Lit::Number(number?)))
            .ok_or("Could not add token.".to_string()); // TODO: More rubbish error

        added_token
    }

    fn scan_to_identifier_then_advance(&mut self) -> Result<TokType, String> {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let value = self
            .source_slice(self.start, self.current)
            .unwrap_or("Could not retrieve source slice.".to_string());

        let token_type = KEYWORDS.get(value.as_str()).cloned();

        if let Some(token_type) = token_type {
            return self
                .add_token(token_type, None)
                .ok_or("Could not add token.".to_string());
        }

        self.add_token(TokType::Identifier, None)
            .ok_or("Could not add token.".to_string())
    }

    fn is_char_then_advance(&mut self, character: char) -> bool {
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

    fn is_at_end(&self) -> bool {
        self.current
            >= self
                .source
                .len()
                .try_into()
                .expect("Unable to convert source length to u64!")
    }

    // TODO: convert from -> \0 to -> Option<char> rather than returning \0, it will be more idiomatic
    /// Peek at the char found at self.current. Does not advance self.current.
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.peek_at(self.current)
    }

    /// Peek at a specific index
    fn peek_at(&self, index: usize) -> char {
        if index >= self.source.len() {
            return '\0';
        }

        self.source[index]
    }

    /// Peek & advance the current index by 1
    fn advance(&mut self) -> Option<char> {
        let character = self.peek();

        if character == '\0' {
            return None;
        }

        self.current += 1;

        Some(character)
    }

    /// Take a slice of the source using a start and end index
    fn source_slice(&self, start: usize, end: usize) -> Option<String> {
        let slice = self.source.get(start..end)?.iter().collect();

        Some(slice)
    }

    fn add_token(&mut self, token_type: TokType, literal_type: Option<Lit>) -> Option<TokType> {
        let start = self.start;
        let current = self.current;
        let lexeme: String = self.source_slice(start, current)?;

        let token = Tok {
            token_type: token_type.clone(),
            lexeme,
            literal: literal_type,
            line: self.line,
        };

        self.tokens.push(token);

        Some(token_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{CRLF, LF},
        types::literal_type::Lit,
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

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 26);
        assert_eq!(scanner.line, 5);
    }

    #[test]
    fn should_increment_line_on_crlf() {
        let source = CRLF.repeat(3);
        let mut scanner = Scanner::new(source.as_ref());

        scanner.scan_tokens().ok();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_increment_line_on_lf() {
        let source = LF.repeat(3);
        let mut scanner = Scanner::new(source.as_ref());

        scanner.scan_tokens().ok();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_match_combo_tokens() {
        let source = "!= <= 60 >= == 123.45";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn should_match_string_literal() {
        let source = "\"hey\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(
            scanner.tokens[0].literal,
            Some(Lit::String("hey".to_string()))
        );
    }

    #[test]
    fn should_match_string_literal_with_other_chars() {
        let source = "\"hey, time to be happy! :)\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(
            scanner.tokens[0].literal,
            Some(Lit::String("hey, time to be happy! :)".to_string()))
        );
    }

    #[test]
    fn should_match_string_literal_wrapped_by_chars() {
        let source = "(\"hey, time to be happy! q:)|=<; \")";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(
            scanner.tokens[1].literal,
            Some(Lit::String("hey, time to be happy! q:)|=<; ".to_string())),
        );
    }

    #[test]
    fn should_match_number() {
        let source = "100";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens[0].literal, Some(Lit::Number(100.0)));
    }

    #[test]
    fn should_match_float_number() {
        let source = "10.1";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens[0].literal, Some(Lit::Number(10.1)));
    }

    #[test]
    fn should_match_identifier() {
        let source = "rando identifier";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens[0].token_type, TokType::Identifier);
        assert_eq!(scanner.tokens[1].token_type, TokType::Identifier);
    }

    #[test]
    fn should_match_embedded_identifier() {
        let source = "rando = identifier + 1";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens[0].token_type, TokType::Identifier);
        assert_eq!(scanner.tokens[2].token_type, TokType::Identifier);
    }

    #[test]
    fn should_match_token_type_from_hash() {
        let source = "var wow = 1 + 1";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens[0].token_type, TokType::Var);
        assert_eq!(scanner.tokens[1].token_type, TokType::Identifier);
    }
}
