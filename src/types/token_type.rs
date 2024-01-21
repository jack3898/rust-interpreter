use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokType {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    None, // Used for internal processing, should not ever live beyond the scanner
}

impl Display for TokType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Self::And => "double-and",
                Self::Bang => "exclamation",
                Self::LeftParen => "left paren",
                Self::RightParen => "right paren",
                Self::LeftBrace => "left brace",
                Self::RightBrace => "right brace",
                Self::Comma => "comma",
                Self::Dot => "full-stop",
                Self::Minus => "minus",
                Self::Plus => "plus",
                Self::Semicolon => "semicolon",
                Self::Slash => "slash",
                Self::Star => "asterisk",
                Self::BangEqual => "not equal",
                Self::Equal => "equal",
                Self::EqualEqual => "double equal",
                Self::Greater => "bigger-than",
                Self::GreaterEqual => "bigger-than or equal",
                Self::Less => "less-than",
                Self::LessEqual => "less-than or equal",
                Self::Identifier => "identifier",
                Self::String => "string",
                Self::Number => "number",
                Self::Class => "class",
                Self::Else => "else",
                Self::False => "false",
                Self::Fun => "function",
                Self::For => "for",
                Self::If => "if",
                Self::Nil => "nil",
                Self::Or => "or",
                Self::Print => "print",
                Self::Return => "return",
                Self::Super => "super",
                Self::This => "this",
                Self::True => "true",
                Self::Var => "variable",
                Self::While => "while",
                Self::Eof => "end of file",
                Self::None => "none",
            }
        );
    }
}
