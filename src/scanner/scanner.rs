use super::{literal_type::LiteralType, token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
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
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, String> {
        while !Self::is_at_end(self) {
            self.start = self.current;
            Self::scan_token(self);
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Option<TokenType> {
        let character = self.advance()?;

        // Single character lexemes
        match character {
            '(' => self.add_token(TokenType::LeftBrace, None),
            ')' => self.add_token(TokenType::RightBrace, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '/' => self.add_token(TokenType::Slash, None), // TODO: Add comment evaluation into this match
            '"' => match self.scan_string_then_advance() {
                Ok(string_token) => Some(string_token),
                Err(error) => {
                    println!("{}", error);
                    None
                }
            },
            '!' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, None)
            }
            '<' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, None)
            }
            '>' => {
                let token_type = if self.is_char_then_advance('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, None)
            }
            '\r' | ' ' => None,
            '\n' => {
                self.line += 1;

                None
            }
            _ => {
                println!(
                    "Error parsing source code at char '{}', line {}",
                    character, self.line
                );

                None
            }
        }
    }

    /// Identify a string of random lengths.
    /// Keeps scanning until it finds the closing quote and will
    /// Respond with the token. Uses self.start and advances self.current to the closing quote to return the string.
    fn scan_string_then_advance(&mut self) -> Result<TokenType, String> {
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
            .add_token(TokenType::String, Some(LiteralType::String(value?)))
            .ok_or("Could not add token.".to_string()); // TODO: More rubbish error

        added_token
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

        self.source[self.current]
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

    fn add_token(
        &mut self,
        token_type: TokenType,
        literal_type: Option<LiteralType>,
    ) -> Option<TokenType> {
        let start = self.start;
        let current = self.current;
        let lexeme: String = self.source_slice(start, current)?;

        let token = Token {
            token_type: token_type.clone(),
            lexeme,
            literal: literal_type,
            line: self.line,
        };

        self.tokens.push(token);

        Some(token_type)
    }
}

mod tests {
    use crate::{
        constants::{CRLF, LF},
        scanner::literal_type::LiteralType,
    };

    use super::{Scanner, TokenType};

    #[test]
    fn should_scan_with_eof() {
        let source = "(";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof)
    }

    #[test]
    fn should_scan_with_token_combo() {
        let source = "(( )) {{ }} , . - + ; * / ! = < >";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 20);
        assert_eq!(scanner.line, 1);
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
        let source = "!= <= >= ==";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn should_match_string_literal() {
        let source = "\"hey\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 2);

        match &scanner.tokens[0].literal {
            Some(LiteralType::String(literal_string)) => {
                assert_eq!(literal_string, "hey");
            }
            _ => panic!("Expected a string literal"),
        }
    }

    #[test]
    fn should_match_string_literal_with_other_chars() {
        let source = "\"hey, time to be happy! :)\"";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 2);

        match &scanner.tokens[0].literal {
            Some(LiteralType::String(literal_string)) => {
                assert_eq!(literal_string, "hey, time to be happy! :)");
            }
            _ => panic!("Expected a string literal"),
        }
    }

    #[test]
    fn should_match_string_literal_wrapped_by_chars() {
        let source = "(\"hey, time to be happy! q:)|=<; \")";
        let mut scanner = Scanner::new(source);

        scanner.scan_tokens().ok();

        assert_eq!(scanner.tokens.len(), 4);

        match &scanner.tokens[1].literal {
            Some(LiteralType::String(literal_string)) => {
                assert_eq!(literal_string, "hey, time to be happy! q:)|=<; ");
            }
            _ => panic!("Expected a string literal"),
        }
    }
}
